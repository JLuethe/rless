#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdarg.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

static void mark(const char *text)
{
    const char *marker = getenv("RLESS_COLLISION_MARKER");
    if (!marker) return;
    int output = open(marker, O_WRONLY | O_CREAT | O_APPEND | O_CLOEXEC, 0600);
    if (output >= 0) { (void)write(output, text, strlen(text)); (void)close(output); }
}

ssize_t getrandom(void *buffer, size_t length, unsigned int flags)
{
    static ssize_t (*original)(void *, size_t, unsigned int);
    if (!original) original = dlsym(RTLD_NEXT, "getrandom");
    if (length == 16 && getenv("RLESS_COLLISION_PROBE")) {
        mark("random\n");
        memset(buffer, 0x42, 16);
        return 16;
    }
    return original(buffer, length, flags);
}

int openat(int directory, const char *path, int flags, ...)
{
    static int (*original)(int, const char *, int, ...);
    if (!original) original = dlsym(RTLD_NEXT, "openat");
    mode_t mode = 0;
    if (flags & (O_CREAT | O_TMPFILE)) {
        va_list arguments;
        va_start(arguments, flags);
        mode = va_arg(arguments, mode_t);
        va_end(arguments);
    }
    if (getenv("RLESS_COLLISION_PROBE") && (flags & O_TMPFILE) == O_TMPFILE) {
        mark("fallback\n");
        errno = EOPNOTSUPP;
        return -1;
    }
    int result = original(directory, path, flags, mode);
    if (getenv("RLESS_COLLISION_PROBE") && result == -1 && errno == EEXIST &&
        path && strncmp(path, ".historyX.", 10) == 0 && (flags & O_EXCL)) {
        int saved = errno;
        mark("open-collision\n");
        (void)kill(getppid(), SIGTERM);
        for (;;) pause();
        errno = saved;
    }
    return result;
}

int fstatat(int directory, const char *path, struct stat *out, int flags)
{
    static int (*original)(int, const char *, struct stat *, int);
    if (!original) original = dlsym(RTLD_NEXT, "fstatat");
    int result = original(directory, path, out, flags);
    if (getenv("RLESS_COLLISION_PROBE") && result == 0 && path &&
        strncmp(path, ".historyX.", 10) == 0) mark("stat-collision\n");
    return result;
}
