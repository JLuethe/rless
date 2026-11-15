/* Test-only whole-ASAN-allocator observation. No allocator replacement, no
 * signal handler, and no object headers or reads before application pointers.
 * Sample immediately before a real blocking terminal read; the parent verifies
 * the READY/input barrier before consuming its latest sample.
 */
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <poll.h>
#include <signal.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <unistd.h>

static int trace_fd = -1;
static pid_t pager_pid;
extern size_t __sanitizer_get_current_allocated_bytes(void);
extern ssize_t __real_read(int, void *, size_t);
extern int __real_ppoll(struct pollfd *, nfds_t, const struct timespec *, const sigset_t *);
static uint64_t sequence;

__attribute__((constructor)) static void configure(void)
{
    const char *text = getenv("RLESS_HEAP_TRACE_FD");
    if (!text || !*text) return;
    char *end;
    errno = 0;
    long value = strtol(text, &end, 10);
    if (*end || errno || value < 3 || value > INT32_MAX) _exit(122);
    trace_fd = (int)value;
    pager_pid = getpid();
    struct stat status;
    int flags = fcntl(trace_fd, F_GETFD);
    if (flags < 0 || fstat(trace_fd, &status) || !S_ISFIFO(status.st_mode) ||
        fcntl(trace_fd, F_SETFD, flags | FD_CLOEXEC)) _exit(122);
    if (unsetenv("RLESS_HEAP_TRACE_FD")) _exit(122);
}

static void sample(int fd, uint64_t call, uint64_t argument)
{
    /* Cancellable read workers inherit this shim but close the pager's trace
     * pipe before entering native read. Only the original pager has an
     * observable live-heap checkpoint; child samples are not part of it. */
    if (trace_fd >= 0 && getpid() == pager_pid && isatty(fd)) {
        if (sequence == UINT64_MAX) _exit(122);
        uint64_t bytes = __sanitizer_get_current_allocated_bytes();
        uint64_t ordinal = ++sequence;
        unsigned char record[48] = {'R', 'H', 'P', '1', 1, 0, 0, 0};
        for (unsigned i = 0; i < 8; ++i) {
            record[8+i] = (unsigned char)(ordinal >> (8*i));
            record[16+i] = (unsigned char)(bytes >> (8*i));
            record[24+i] = (unsigned char)((uint64_t)fd >> (8*i));
            record[32+i] = (unsigned char)(call >> (8*i));
            record[40+i] = (unsigned char)(argument >> (8*i));
        }
        ssize_t result;
        do { result = syscall(SYS_write, trace_fd, record, sizeof record); }
        while (result < 0 && errno == EINTR);
        if (result != sizeof record) _exit(124);
    }
}

ssize_t __wrap_read(int fd, void *buffer, size_t count)
{
    int saved_errno = errno;
    sample(fd, SYS_read, (uint64_t)fd);
    errno = saved_errno;
    return __real_read(fd, buffer, count);
}

int __wrap_ppoll(struct pollfd *fds, nfds_t count, const struct timespec *timeout, const sigset_t *mask)
{
    int saved_errno = errno;
    /* runtime::read_interruptible waits on exactly one input descriptor. */
    if (count == 1 && fds && (fds[0].events & POLLIN) && !timeout)
        sample(fds[0].fd, SYS_ppoll, (uintptr_t)fds);
    errno = saved_errno;
    return __real_ppoll(fds, count, timeout, mask);
}
