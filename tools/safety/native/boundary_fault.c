/* Test-only faults at the pager's log, terminal, descriptor and history APIs. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/syscall.h>
#include <termios.h>
#include <unistd.h>

static int fired;
static int terminal_calls;
static int swaps;
static int tty_opens;
static FILE *history_stream;

static int plan(const char *name) {
    const char *value = getenv("RLESS_BOUNDARY_PLAN");
    return value && strcmp(value, name) == 0;
}

static void hit(void) {
    if (fired++) return;
    const char *path = getenv("RLESS_BOUNDARY_TRACE");
    if (!path) _exit(123);
    int fd = syscall(SYS_openat, AT_FDCWD, path, O_WRONLY | O_APPEND | O_CLOEXEC, 0);
    if (fd < 0 || syscall(SYS_write, fd, "hit\n", 4) != 4) _exit(124);
    syscall(SYS_close, fd);
}

static int named(int fd, const char *suffix) {
    char proc[64], path[1024];
    snprintf(proc, sizeof proc, "/proc/self/fd/%d", fd);
    ssize_t n = syscall(SYS_readlinkat, AT_FDCWD, proc, path, sizeof path - 1);
    if (n < 0) return 0;
    path[n] = 0;
    const char *base = strrchr(path, '/');
    base = base ? base + 1 : path;
    return strcmp(base, suffix) == 0;
}

ssize_t write(int fd, const void *data, size_t count) {
    ssize_t (*real_call)(int, const void *, size_t) = dlsym(RTLD_NEXT, "write");
    if (!real_call) _exit(125);
    if (named(fd, "capture")) {
        if (plan("log-short") && !fired && count > 3) { hit(); return real_call(fd, data, 3); }
        if (plan("log-eintr") && !fired) { hit(); errno = EINTR; return -1; }
        if (plan("log-eio") || plan("log-enospc")) {
            hit(); errno = plan("log-eio") ? EIO : ENOSPC; return -1;
        }
    }
    return real_call(fd, data, count);
}

int close(int fd) {
    int (*real_call)(int) = dlsym(RTLD_NEXT, "close");
    if (!real_call) _exit(125);
    int fail = named(fd, "capture") && plan("log-close");
    int result = real_call(fd);
    if (fail) { hit(); errno = EIO; return -1; }
    return result;
}

int tcsetattr(int fd, int action, const struct termios *term) {
    int (*real_call)(int, int, const struct termios *) = dlsym(RTLD_NEXT, "tcsetattr");
    if (!real_call) _exit(125);
    ++terminal_calls;
    if (plan("term-restore") && terminal_calls >= 2) {
        hit(); errno = EIO; return -1;
    }
    return real_call(fd, action, term);
}

int fcntl(int fd, int command, ...) {
    int (*real_call)(int, int, ...) = dlsym(RTLD_NEXT, "fcntl");
    if (!real_call) _exit(125);
    if (fd == 0 && command == F_DUPFD_CLOEXEC && plan("fd-backup")) {
        hit(); errno = EMFILE; return -1;
    }
    long arg = 0;
    if (command != F_GETFD && command != F_GETFL && command != F_GETOWN) {
        va_list args;
        va_start(args, command);
        arg = va_arg(args, long);
        va_end(args);
    }
    return real_call(fd, command, arg);
}

int dup2(int oldfd, int newfd) {
    int (*real_call)(int, int) = dlsym(RTLD_NEXT, "dup2");
    if (!real_call) _exit(125);
    if (newfd == 0 && oldfd != 0) {
        ++swaps;
        if (plan("fd-swap-eintr") && swaps == 1) { hit(); errno = EINTR; return -1; }
        if (plan("fd-swap-eio") && swaps == 1) { hit(); errno = EIO; return -1; }
        if (plan("fd-restore-eio") && swaps >= 2) { hit(); errno = EIO; return -1; }
    }
    return real_call(oldfd, newfd);
}

int open(const char *path, int flags, ...) {
    int (*real_call)(const char *, int, ...) = dlsym(RTLD_NEXT, "open");
    if (!real_call) _exit(125);
    mode_t mode = 0;
    if ((flags & O_CREAT) || (flags & O_TMPFILE) == O_TMPFILE) {
        va_list args;
        va_start(args, flags);
        mode = va_arg(args, int);
        va_end(args);
    }
    if (plan("fd-tty-owned-2") &&
        (strcmp(path, "/dev/tty") == 0 || strncmp(path, "/dev/pts/", 9) == 0) &&
        ++tty_opens == 2) {
        syscall(SYS_close, 2);
        int fd = real_call(path, flags, mode);
        if (fd != 2) _exit(125);
        hit();
        return fd;
    }
    if (plan("fd-tty-fallback") &&
        (strcmp(path, "/dev/tty") == 0 || strncmp(path, "/dev/pts/", 9) == 0) &&
        ++tty_opens >= 2) {
        hit(); errno = EACCES; return -1;
    }
    return real_call(path, flags, mode);
}

FILE *fdopen(int fd, const char *mode) {
    FILE *(*real_call)(int, const char *) = dlsym(RTLD_NEXT, "fdopen");
    if (!real_call) _exit(125);
    /* The randomized basename starts with history. */
    {
        char proc[64], path[1024];
        snprintf(proc, sizeof proc, "/proc/self/fd/%d", fd);
        ssize_t n = syscall(SYS_readlinkat, AT_FDCWD, proc, path, sizeof path - 1);
        if (n >= 0) {
            path[n] = 0;
            const char *base = strrchr(path, '/');
            if (base && strncmp(base + 1, "history.", 8) == 0) {
                if (plan("hist-fdopen")) { hit(); errno = EMFILE; return NULL; }
                history_stream = real_call(fd, mode);
                if (history_stream && plan("hist-write")) {
                    /* A real stdio write to /dev/full sets the stream's error
                     * flag; merely returning -1 from a fprintf shim would not. */
                    if (setvbuf(history_stream, NULL, _IONBF, 0)) _exit(126);
                    int full = syscall(SYS_openat, AT_FDCWD, "/dev/full", O_WRONLY | O_CLOEXEC, 0);
                    if (full < 0 || syscall(SYS_dup2, full, fd) != fd) _exit(126);
                    syscall(SYS_close, full);
                    hit();
                }
                return history_stream;
            }
        }
    }
    return real_call(fd, mode);
}

int ferror(FILE *stream) {
    int (*real_call)(FILE *) = dlsym(RTLD_NEXT, "ferror");
    if (!real_call) _exit(125);
    if (plan("hist-ferror") && stream == history_stream) { hit(); return 1; }
    return real_call(stream);
}

int fclose(FILE *stream) {
    int (*real_call)(FILE *) = dlsym(RTLD_NEXT, "fclose");
    if (!real_call) _exit(125);
    int fail = plan("hist-fclose") && stream == history_stream;
    int result = real_call(stream);
    if (fail) { hit(); errno = EIO; return -1; }
    return result;
}

int rename(const char *source, const char *dest) {
    int (*real_call)(const char *, const char *) = dlsym(RTLD_NEXT, "rename");
    if (!real_call) _exit(125);
    if (plan("hist-rename") && strstr(source, "history.") &&
        strcmp(strrchr(dest, '/') ? strrchr(dest, '/') + 1 : dest, "history") == 0) {
        hit(); errno = EIO; return -1;
    }
    return real_call(source, dest);
}
