#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <poll.h>
#include <stdarg.h>
#include <stdlib.h>
#include <string.h>
#include <sys/syscall.h>
#include <unistd.h>

int openat(int directory, const char *path, int flags, ...)
{
    mode_t mode = 0;
    if (flags & (O_CREAT | O_TMPFILE)) {
        va_list args;
        va_start(args, flags);
        mode = va_arg(args, mode_t);
        va_end(args);
    }
    if ((flags & O_TMPFILE) == O_TMPFILE) {
        errno = EOPNOTSUPP;
        return -1;
    }
    return (int)syscall(SYS_openat, directory, path, flags, mode);
}

int poll(struct pollfd *fds, nfds_t count, int timeout)
{
    if (getenv("RLESS_SKIP_CLEANUP_WAIT") && count == 1 && timeout == -1
        && fds[0].fd == 5) {
        char target[64];
        ssize_t size = readlink("/proc/self/fd/5", target, sizeof target - 1);
        if (size > 0) {
            target[size] = 0;
            if (strstr(target, "pidfd")) {
                const char *marker = getenv("RLESS_SKIP_MARKER");
                if (marker) {
                    int fd = open(marker, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
                    if (fd >= 0) { (void)write(fd, "hit\n", 4); (void)close(fd); }
                }
                fds[0].revents = POLLIN;
                return 1;
            }
        }
    }
    return (int)syscall(SYS_poll, fds, count, timeout);
}
