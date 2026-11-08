/* Deliver SIGTERM immediately before the binary-file probe's blocking step. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <poll.h>
#include <signal.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/socket.h>
#include <sys/syscall.h>
#include <time.h>
#include <unistd.h>

static int fired;
static pid_t initial_pid;
static int *worker_read_entered;

__attribute__((constructor)) static void initialize(void) {
    initial_pid = getpid();
    worker_read_entered = mmap(NULL, sizeof *worker_read_entered,
                               PROT_READ | PROT_WRITE, MAP_SHARED | MAP_ANONYMOUS,
                               -1, 0);
    if (worker_read_entered == MAP_FAILED) _exit(127);
}

static int probe_file(int fd) {
    char proc[64], path[1024];
    snprintf(proc, sizeof proc, "/proc/self/fd/%d", fd);
    ssize_t n = syscall(SYS_readlinkat, AT_FDCWD, proc, path, sizeof path - 1);
    if (n < 0) return 0;
    path[n] = 0;
    const char *base = strrchr(path, '/');
    return strcmp(base ? base + 1 : path, "regular-input") == 0;
}

static void mark(char kind) {
    const char *trace = getenv("RLESS_BIN_FILE_SIGNAL_TRACE");
    if (!trace) _exit(123);
    int fd = syscall(SYS_openat, AT_FDCWD, trace,
                     O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
    if (fd < 0 || syscall(SYS_write, fd, &kind, 1) != 1) _exit(124);
    syscall(SYS_close, fd);
}

ssize_t read(int fd, void *buffer, size_t length) {
    if (!fired && length == 256 && probe_file(fd)) {
        fired = 1;
        if (getpid() == initial_pid) {
            mark('R');
            /* The legacy probe has no atomic wait. Delivery here is complete
             * before its next read starts, leaving the later read asleep. */
            syscall(SYS_tgkill, getpid(), syscall(SYS_gettid), SIGTERM);
        } else {
            /* The path-backed worker owns this read; the parent will deliver
             * the signal after the read request has entered this stall. */
            __atomic_store_n(worker_read_entered, 1, __ATOMIC_RELEASE);
        }
        int pipefd[2];
        if (syscall(SYS_pipe2, pipefd, O_CLOEXEC) != 0) _exit(125);
        return syscall(SYS_read, pipefd[0], buffer, 1);
    }
    return syscall(SYS_read, fd, buffer, length);
}

ssize_t send(int fd, const void *buffer, size_t length, int flags) {
    ssize_t (*next)(int, const void *, size_t, int) = dlsym(RTLD_NEXT, "send");
    if (!next) _exit(126);
    ssize_t result = next(fd, buffer, length, flags);
    if (getpid() == initial_pid && result == (ssize_t)length && length == 24 &&
        !fired && buffer) {
        uint32_t command, count;
        memcpy(&command, buffer, sizeof command);
        memcpy(&count, (const char *)buffer + sizeof command, sizeof count);
        if (command == 1 && count == 256) {
            struct timespec step = {0, 1000000};
            for (int attempt = 0; attempt < 1000 &&
                 !__atomic_load_n(worker_read_entered, __ATOMIC_ACQUIRE);
                 ++attempt) nanosleep(&step, NULL);
            if (__atomic_load_n(worker_read_entered, __ATOMIC_ACQUIRE)) {
                fired = 1;
                mark('P');
                syscall(SYS_tgkill, getpid(), syscall(SYS_gettid), SIGTERM);
            }
        }
    }
    return result;
}

int ppoll(struct pollfd *fds, nfds_t count, const struct timespec *timeout,
          const sigset_t *mask) {
    int (*next)(struct pollfd *, nfds_t, const struct timespec *, const sigset_t *) =
        dlsym(RTLD_NEXT, "ppoll");
    if (!next) _exit(126);
    if (!fired && count == 1 && fds && probe_file(fds[0].fd)) {
        fired = 1;
        mark('P');
        syscall(SYS_tgkill, getpid(), syscall(SYS_gettid), SIGTERM);
    }
    return next(fds, count, timeout, mask);
}
