/* Test-only native short-write counterpart; never packaged with the pager. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <unistd.h>

struct shared_short_state {
    unsigned long attempts;
    unsigned long pending_ordinal;
};

static struct shared_short_state *shared;
static pid_t initial_pid;
static int output_fds[1024];

__attribute__((constructor)) static void initialize_shared_state(void)
{
    shared = mmap(NULL, sizeof *shared, PROT_READ | PROT_WRITE,
                  MAP_SHARED | MAP_ANONYMOUS, -1, 0);
    if (shared == MAP_FAILED) _exit(125);
    initial_pid = getpid();
}

static void emit_marker(unsigned long ordinal)
{
    ssize_t (*real_call)(int, const void *, size_t) = dlsym(RTLD_NEXT, "write");
    const char *trace = getenv("RLESS_NATIVE_SHORT_WRITE_FD");
    if (!real_call || !trace) _exit(125);
    unsigned char marker[16] = {
        'R', 'N', 'T', '1', 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
    };
    for (unsigned i = 0; i < 8; ++i)
        marker[8+i] = (unsigned char)((unsigned long long)ordinal >> (8*i));
    if (real_call(atoi(trace), marker, sizeof marker) != sizeof marker) _exit(124);
}

int dup3(int source, int target, int flags)
{
    int (*real_call)(int, int, int) = dlsym(RTLD_NEXT, "dup3");
    if (!real_call) _exit(125);
    int result = real_call(source, target, flags);
    if (result >= 0 && target >= 0 && target < 1024) {
        int output = source == STDOUT_FILENO ||
            (source >= 0 && source < 1024 && output_fds[source]);
        if (!output && getpid() != initial_pid) {
            struct stat from, stdout_info;
            if (fstat(source, &from) == 0 && fstat(STDOUT_FILENO, &stdout_info) == 0)
                output = from.st_mode == stdout_info.st_mode &&
                    from.st_dev == stdout_info.st_dev && from.st_ino == stdout_info.st_ino &&
                    from.st_rdev == stdout_info.st_rdev;
        }
        output_fds[target] = output;
    }
    return result;
}

ssize_t recvmsg(int fd, struct msghdr *message, int flags)
{
    ssize_t (*real_call)(int, struct msghdr *, int) = dlsym(RTLD_NEXT, "recvmsg");
    if (!real_call) _exit(125);
    ssize_t result = real_call(fd, message, flags);
    if (result >= 0) {
        unsigned long ordinal = __atomic_exchange_n(&shared->pending_ordinal, 0, __ATOMIC_ACQUIRE);
        if (ordinal) emit_marker(ordinal);
    }
    return result;
}

/* A private nonblocking pipe writer has a different fd but the same kernel
 * pipe inode as stdout. Follow that verified identity when selecting the
 * production write boundary; the trace pipe has a different inode. */
static int output_fd(int fd)
{
    if (fd == STDOUT_FILENO) return 1;
    if (fd < 0) return 0;
    if (fd < 1024 && output_fds[fd]) return 1;
    struct stat stdout_info, target_info;
    return fstat(STDOUT_FILENO, &stdout_info) == 0
        && fstat(fd, &target_info) == 0
        && (stdout_info.st_mode & S_IFMT) == (target_info.st_mode & S_IFMT)
        && stdout_info.st_dev == target_info.st_dev
        && stdout_info.st_ino == target_info.st_ino
        && stdout_info.st_rdev == target_info.st_rdev;
}

ssize_t write(int fd, const void *buffer, size_t count)
{
    ssize_t (*real_call)(int, const void *, size_t) = dlsym(RTLD_NEXT, "write");
    if (!real_call) _exit(125);
    const char *trace = getenv("RLESS_NATIVE_SHORT_WRITE_FD");
    const char *plan = getenv("RLESS_NATIVE_SHORT_WRITE_AT");
    unsigned long ordinal = plan ? strtoul(plan, NULL, 10) : 1;
    if (output_fd(fd) && count > 1 && trace &&
        __atomic_add_fetch(&shared->attempts, 1, __ATOMIC_SEQ_CST) == ordinal) {
        if (getpid() == initial_pid) emit_marker(ordinal);
        else __atomic_store_n(&shared->pending_ordinal, ordinal, __ATOMIC_RELEASE);
        return real_call(fd, buffer, count / 2);
    }
    return real_call(fd, buffer, count);
}
