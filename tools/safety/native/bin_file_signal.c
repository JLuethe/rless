/* Deliver SIGTERM immediately before the binary-file probe's blocking step. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <poll.h>
#include <signal.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/syscall.h>
#include <time.h>
#include <unistd.h>

static int fired;

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
        mark('R');
        /* The legacy probe has no atomic wait. Delivery here is complete
         * before its next read starts, leaving the later read asleep. */
        syscall(SYS_tgkill, getpid(), syscall(SYS_gettid), SIGTERM);
        int pipefd[2];
        if (syscall(SYS_pipe2, pipefd, O_CLOEXEC) != 0) _exit(125);
        return syscall(SYS_read, pipefd[0], buffer, 1);
    }
    return syscall(SYS_read, fd, buffer, length);
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
