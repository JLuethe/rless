#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>

static int unnamed_temp_fd = -1;
static volatile int *already_stalled;

__attribute__((constructor)) static void initialize_stall(void)
{
    void *state = mmap(NULL, sizeof(int), PROT_READ | PROT_WRITE,
                       MAP_SHARED | MAP_ANONYMOUS, -1, 0);
    if (state == MAP_FAILED) _exit(125);
    already_stalled = state;
}

static void mark_and_stall(const char *trace)
{
    if (__sync_lock_test_and_set(already_stalled, 1)) return;
    int marker = open(trace, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
    if (marker >= 0) {
        (void)write(marker, "hit\n", 4);
        (void)close(marker);
    }
    for (;;) pause();
}

int openat(int directory, const char *path, int flags, ...)
{
    int (*real_openat)(int, const char *, int, ...) = dlsym(RTLD_NEXT, "openat");
    if (!real_openat) _exit(125);
    mode_t mode = 0;
    if (flags & O_CREAT || (flags & O_TMPFILE) == O_TMPFILE) {
        va_list arguments;
        va_start(arguments, flags);
        mode = va_arg(arguments, mode_t);
        va_end(arguments);
    }
    if (getenv("RLESS_HISTORY_NO_TMPFILE") && (flags & O_TMPFILE) == O_TMPFILE) {
        errno = EOPNOTSUPP;
        return -1;
    }
    int fd = real_openat(directory, path, flags, mode);
    if (fd >= 0 && (flags & O_TMPFILE) == O_TMPFILE)
        unnamed_temp_fd = fd;
    const char *stage = getenv("RLESS_HISTORY_STALL_STAGE");
    const char *trace = getenv("RLESS_HISTORY_SYNC_TRACE");
    if (fd >= 0 && stage && strcmp(stage, "created") == 0 && trace
        && ((flags & O_TMPFILE) == O_TMPFILE || strstr(path, ".historyX."))) {
        mark_and_stall(trace);
    }
    return fd;
}

int linkat(int olddir, const char *oldpath, int newdir, const char *newpath, int flags)
{
    int (*real_linkat)(int, const char *, int, const char *, int) = dlsym(RTLD_NEXT, "linkat");
    if (!real_linkat) _exit(125);
    int result = real_linkat(olddir, oldpath, newdir, newpath, flags);
    const char *stage = getenv("RLESS_HISTORY_STALL_STAGE");
    const char *trace = getenv("RLESS_HISTORY_SYNC_TRACE");
    if (result == 0 && stage && strcmp(stage, "linked") == 0 && trace
        && (flags & AT_EMPTY_PATH) && strstr(newpath, ".historyX."))
        mark_and_stall(trace);
    return result;
}

int fsync(int fd)
{
    int (*real_fsync)(int) = dlsym(RTLD_NEXT, "fsync");
    if (!real_fsync) _exit(125);
    const char *trace = getenv("RLESS_HISTORY_SYNC_TRACE");
    const char *stage = getenv("RLESS_HISTORY_STALL_STAGE");
    if (trace && stage && strcmp(stage, "directory") == 0) {
        struct stat metadata;
        if (fstat(fd, &metadata) == 0 && S_ISDIR(metadata.st_mode))
            mark_and_stall(trace);
    }
    if (trace && (!stage || strcmp(stage, "fsync") == 0)) {
        if (fd == unnamed_temp_fd) mark_and_stall(trace);
        char proc[64];
        char target[512];
        int length = snprintf(proc, sizeof proc, "/proc/self/fd/%d", fd);
        if (length > 0 && (size_t)length < sizeof proc) {
            ssize_t count = readlink(proc, target, sizeof target - 1);
            if (count > 0) {
                target[count] = '\0';
                if (strstr(target, "/.historyX.")) {
                    mark_and_stall(trace);
                }
            }
        }
    }
    return real_fsync(fd);
}
