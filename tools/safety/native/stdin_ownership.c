/* Test-only close witness. Raw syscalls keep logging independent of close(). */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/syscall.h>
#include <unistd.h>
static int witness = -1;
__attribute__((constructor)) static void configure(void) {
    const char *path = getenv("RLESS_STDIN_CLOSE_TRACE");
    if (!path) return;
    witness = syscall(SYS_openat, AT_FDCWD, path, O_WRONLY|O_APPEND|O_CLOEXEC, 0);
    if (witness < 3 || syscall(SYS_write, witness, "ready\n", 6) != 6) _exit(123);
}
int close(int fd) {
    int saved = errno;
    int (*next)(int) = dlsym(RTLD_NEXT, "close");
    if (!next) _exit(125);
    if (witness >= 0 && fd >= 0 && fd <= 2) {
        char text[] = "close 0\n";
        text[6] = '0' + fd;
        if (syscall(SYS_write, witness, text, 8) != 8) _exit(124);
    }
    errno = saved;
    return next(fd);
}
