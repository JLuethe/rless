#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

int open(const char *path, int flags, ...)
{
    int (*next_open)(const char *, int, ...) = dlsym(RTLD_NEXT, "open");
    if (!next_open) _exit(127);
    const char *device = getenv("RLESS_STALL_REOPEN_DEV");
    const char *inode = getenv("RLESS_STALL_REOPEN_INO");
    const char *marker = getenv("RLESS_STALL_REOPEN_MARKER");
    const char *release = getenv("RLESS_STALL_REOPEN_RELEASE");
    int fd = -1;
    if (strcmp(path, "/proc/self/fd/0") == 0) fd = 0;
    if (strcmp(path, "/proc/self/fd/3") == 0) fd = 3;
    if (fd >= 0 && device && inode && marker && release) {
        struct stat status;
        if (fstat(fd, &status) == 0 && S_ISFIFO(status.st_mode)
            && (unsigned long long)status.st_dev == strtoull(device, NULL, 10)
            && (unsigned long long)status.st_ino == strtoull(inode, NULL, 10)) {
            int output = next_open(marker, O_WRONLY | O_CREAT | O_APPEND | O_CLOEXEC, 0600);
            if (output >= 0) {
                char line[64];
                int length = snprintf(line, sizeof(line), "%ld\n", (long)getpid());
                if (length > 0 && length < (int)sizeof(line))
                    (void)write(output, line, (size_t)length);
                (void)close(output);
            }
            while (access(release, F_OK) != 0) {
                struct timespec delay = {0, 10000000};
                (void)nanosleep(&delay, NULL);
            }
        }
    }
    if (flags & O_CREAT) {
        va_list args;
        va_start(args, flags);
        mode_t mode = (mode_t)va_arg(args, int);
        va_end(args);
        return next_open(path, flags, mode);
    }
    return next_open(path, flags);
}
