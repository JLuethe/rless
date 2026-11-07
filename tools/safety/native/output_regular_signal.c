/* Test-only interruption at the regular-file stdout write boundary. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
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
    const char *trace = getenv("RLESS_REGULAR_SIGNAL_TRACE_FD");
    struct stat info;
    if (fd == STDOUT_FILENO && count != 0 && trace && !fired
        && fstat(fd, &info) == 0 && S_ISREG(info.st_mode)) {
        fired = 1;
        const char marker[] = "hit\n";
        if (syscall(SYS_write, atoi(trace), marker, sizeof marker - 1)
            != (ssize_t)(sizeof marker - 1)) _exit(124);
        if (raise(SIGTERM) != 0) _exit(123);
        errno = EINTR;
        return -1;
    }
    return real_write(fd, buffer, count);
}
