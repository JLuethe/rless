#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

int isatty(int fd)
{
    int (*next_isatty)(int) = dlsym(RTLD_NEXT, "isatty");
    if (!next_isatty) _exit(127);
    const char *device = getenv("RLESS_STALL_ISATTY_RDEV");
    const char *marker = getenv("RLESS_STALL_ISATTY_MARKER");
    const char *release = getenv("RLESS_STALL_ISATTY_RELEASE");
    if (device && marker && release) {
        struct stat status;
        if (fstat(fd, &status) == 0 && S_ISCHR(status.st_mode)
            && (unsigned long long)status.st_rdev == strtoull(device, NULL, 10)) {
            int output = open(marker, O_WRONLY | O_CREAT | O_APPEND | O_CLOEXEC, 0600);
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
    return next_isatty(fd);
}
