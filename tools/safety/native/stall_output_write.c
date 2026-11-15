#define _GNU_SOURCE
#include <dlfcn.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

ssize_t write(int fd, const void *buffer, size_t length)
{
    ssize_t (*next_write)(int, const void *, size_t) = dlsym(RTLD_NEXT, "write");
    int (*next_open)(const char *, int, ...) = dlsym(RTLD_NEXT, "open");
    if (!next_write || !next_open) _exit(127);
    const char *device = getenv("RLESS_STALL_OUTPUT_DEV");
    const char *inode = getenv("RLESS_STALL_OUTPUT_INO");
    const char *marker = getenv("RLESS_STALL_OUTPUT_MARKER");
    const char *release = getenv("RLESS_STALL_OUTPUT_RELEASE");
    const char *kind = getenv("RLESS_STALL_OUTPUT_KIND");
    if (device && inode && marker && release) {
        struct stat status;
        if (fstat(fd, &status) == 0
            && ((kind && strcmp(kind, "regular") == 0 && S_ISREG(status.st_mode))
                || ((!kind || strcmp(kind, "char") == 0) && S_ISCHR(status.st_mode)))
            && (unsigned long long)status.st_dev == strtoull(device, NULL, 10)
            && (unsigned long long)status.st_ino == strtoull(inode, NULL, 10)) {
            int output = next_open(marker, O_WRONLY | O_CREAT | O_APPEND | O_CLOEXEC, 0600);
            if (output >= 0) {
                char line[64];
                int count = snprintf(line, sizeof(line), "%ld\n", (long)getpid());
                if (count > 0 && count < (int)sizeof(line))
                    (void)next_write(output, line, (size_t)count);
                (void)close(output);
            }
            while (access(release, F_OK) != 0) {
                struct timespec delay = {0, 10000000};
                (void)nanosleep(&delay, NULL);
            }
        }
    }
    return next_write(fd, buffer, length);
}
