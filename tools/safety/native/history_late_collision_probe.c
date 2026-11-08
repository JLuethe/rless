#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdarg.h>
#include <stdatomic.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <unistd.h>

static atomic_int inserted;

static void mark(const char *text)
{
    const char *path = getenv("RLESS_LATE_COLLISION_MARKER");
    if (!path) return;
    int fd = (int)syscall(SYS_openat, AT_FDCWD, path,
                          O_WRONLY | O_CREAT | O_APPEND | O_CLOEXEC, 0600);
    if (fd >= 0) { (void)write(fd, text, strlen(text)); (void)close(fd); }
}

int openat(int directory, const char *path, int flags, ...)
{
    static int (*original)(int, const char *, int, ...);
    if (!original) original = dlsym(RTLD_NEXT, "openat");
    mode_t mode = 0;
    if (flags & (O_CREAT | O_TMPFILE)) {
        va_list args;
        va_start(args, flags);
        mode = va_arg(args, mode_t);
        va_end(args);
    }
    if (getenv("RLESS_LATE_COLLISION_MARKER") &&
        (flags & O_TMPFILE) == O_TMPFILE) {
        mark("fallback\n");
        errno = EOPNOTSUPP;
        return -1;
    }
    int result = original(directory, path, flags, mode);
    if (getenv("RLESS_LATE_COLLISION_MARKER") && result == -1 &&
        errno == EEXIST && strncmp(path, ".historyX.", 10) == 0 &&
        (flags & O_EXCL)) {
        mark("open-collision\n");
        (void)kill(getppid(), SIGTERM);
        for (;;) pause();
    }
    return result;
}

int fstatat(int directory, const char *path, struct stat *out, int flags)
{
    static int (*original)(int, const char *, struct stat *, int);
    if (!original) original = dlsym(RTLD_NEXT, "fstatat");
    int result = original(directory, path, out, flags);
    if (getenv("RLESS_LATE_COLLISION_MARKER") && result == -1 &&
        errno == ENOENT && strncmp(path, ".historyX.", 10) == 0 &&
        atomic_exchange(&inserted, 1) == 0) {
        int fd = (int)syscall(SYS_openat, directory, path,
                              O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC, 0600);
        if (fd >= 0) {
            (void)write(fd, "unrelated\n", 10);
            (void)close(fd);
            mark("late-created\n");
        }
        errno = ENOENT;
    }
    return result;
}
