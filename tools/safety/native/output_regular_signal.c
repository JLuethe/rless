/* Test-only interruption at the regular-file stdout write boundary. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <unistd.h>

ssize_t write(int fd, const void *buffer, size_t count)
{
    static int fired;
    ssize_t (*real_write)(int, const void *, size_t) = dlsym(RTLD_NEXT, "write");
    if (!real_write) _exit(125);
    const char *marker_path = getenv("RLESS_REGULAR_SIGNAL_MARKER");
    struct stat info;
    if ((fd == STDOUT_FILENO || fd == 3) && count != 0 && marker_path && !fired
        && fstat(fd, &info) == 0 && S_ISREG(info.st_mode)) {
        fired = 1;
        int marker_fd = open(marker_path, O_WRONLY | O_CREAT | O_CLOEXEC, 0600);
        if (marker_fd < 0) _exit(124);
        const char marker_bytes[] = "hit\n";
        if (syscall(SYS_write, marker_fd, marker_bytes, sizeof marker_bytes - 1)
            != (ssize_t)(sizeof marker_bytes - 1)) _exit(124);
        close(marker_fd);
        if (fd == STDOUT_FILENO) {
            if (raise(SIGTERM) != 0) _exit(123);
        } else if (kill(getppid(), SIGTERM) != 0) {
            _exit(123);
        }
        errno = EINTR;
        return -1;
    }
    return real_write(fd, buffer, count);
}
