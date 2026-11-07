#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

static void report_and_wait(const char *marker, const char *release,
                            int whence, off_t offset)
{
    int output = open(marker, O_WRONLY | O_CREAT | O_APPEND | O_CLOEXEC, 0600);
    if (output >= 0) {
        char line[96];
        int length = snprintf(line, sizeof(line), "%ld %d %lld\n",
                              (long)getpid(), whence, (long long)offset);
        if (length > 0 && length < (int)sizeof(line))
            (void)write(output, line, (size_t)length);
        (void)close(output);
    }
    while (access(release, F_OK) != 0) {
        struct timespec delay = {0, 10000000};
        (void)nanosleep(&delay, NULL);
    }
}

off_t lseek(int fd, off_t offset, int whence)
{
    off_t (*next_lseek)(int, off_t, int) = dlsym(RTLD_NEXT, "lseek");
    if (!next_lseek) _exit(127);
    const char *device = getenv("RLESS_STALL_SEEK_DEV");
    const char *inode = getenv("RLESS_STALL_SEEK_INO");
    const char *marker = getenv("RLESS_STALL_SEEK_MARKER");
    const char *release = getenv("RLESS_STALL_SEEK_RELEASE");
    const char *after_commit = getenv("RLESS_STALL_SEEK_AFTER_COMMIT");
    int hold = 0;
    if (device && inode && marker && release) {
        struct stat status;
        if (fstat(fd, &status) == 0 && S_ISREG(status.st_mode)
            && (unsigned long long)status.st_dev == strtoull(device, NULL, 10)
            && (unsigned long long)status.st_ino == strtoull(inode, NULL, 10)) {
            hold = !after_commit || (whence == SEEK_SET && offset >= 8192);
        }
    }
    if (hold && !after_commit) report_and_wait(marker, release, whence, offset);
    off_t result = next_lseek(fd, offset, whence);
    if (hold && after_commit && result >= 0)
        report_and_wait(marker, release, whence, offset);
    return result;
}
