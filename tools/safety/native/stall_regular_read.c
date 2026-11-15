/* Force a regular-file read to block after readiness, retaining the caller mask. */
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/syscall.h>
#include <unistd.h>

static int fired;

static int target(int fd) {
    char proc[64], path[1024];
    snprintf(proc, sizeof proc, "/proc/self/fd/%d", fd);
    ssize_t n = syscall(SYS_readlinkat, AT_FDCWD, proc, path, sizeof path - 1);
    if (n < 0) return 0;
    path[n] = 0;
    const char *base = strrchr(path, '/');
    const char *wanted = getenv("RLESS_REGULAR_READ_TARGET");
    if (!wanted) wanted = "regular-input";
    return strcmp(base ? base + 1 : path, wanted) == 0;
}

static ssize_t stall_or_read(int fd, void *buffer, size_t length,
                             int positioned, off_t offset) {
    if (length && !fired && target(fd)) {
        sigset_t mask;
        if (sigprocmask(SIG_SETMASK, NULL, &mask) != 0) _exit(121);
        int blocked = sigismember(&mask, SIGTERM);
        const char *mode = getenv("RLESS_REGULAR_READ_MODE");
        int control = mode && strcmp(mode, "unmasked") == 0;
        if (control && blocked) {
            sigset_t term;
            if (sigemptyset(&term) != 0 || sigaddset(&term, SIGTERM) != 0 ||
                sigprocmask(SIG_UNBLOCK, &term, NULL) != 0) _exit(126);
            blocked = 0;
        }
        if (blocked != (control ? 0 : 1)) {
            return positioned ? syscall(SYS_pread64, fd, buffer, length, offset)
                              : syscall(SYS_read, fd, buffer, length);
        }
        fired = 1;
        const char *trace = getenv("RLESS_REGULAR_READ_TRACE");
        if (!trace || blocked < 0) _exit(122);
        int output = syscall(SYS_openat, AT_FDCWD, trace,
                             O_WRONLY | O_CREAT | O_CLOEXEC |
                             (mode && strcmp(mode, "once") == 0 ? O_EXCL : O_TRUNC), 0600);
        if (output < 0 && errno == EEXIST && mode && strcmp(mode, "once") == 0) {
            return positioned ? syscall(SYS_pread64, fd, buffer, length, offset)
                              : syscall(SYS_read, fd, buffer, length);
        }
        if (output < 0) _exit(123);
        char state = blocked ? 'B' : 'U';
        if (syscall(SYS_write, output, &state, 1) != 1) _exit(124);
        syscall(SYS_close, output);
        int pipefd[2];
        if (syscall(SYS_pipe2, pipefd, O_CLOEXEC) != 0) _exit(125);
        /* The open writer prevents EOF. With SIGTERM masked this blocks until
           the harness sends SIGKILL; with it unmasked read returns EINTR. */
        return syscall(SYS_read, pipefd[0], buffer, 1);
    }
    return positioned ? syscall(SYS_pread64, fd, buffer, length, offset)
                      : syscall(SYS_read, fd, buffer, length);
}

ssize_t read(int fd, void *buffer, size_t length) {
    return stall_or_read(fd, buffer, length, 0, 0);
}

ssize_t pread(int fd, void *buffer, size_t length, off_t offset) {
    return stall_or_read(fd, buffer, length, 1, offset);
}
