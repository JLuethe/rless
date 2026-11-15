#define _GNU_SOURCE
#include <dlfcn.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdlib.h>
#include <sys/ioctl.h>
#include <unistd.h>

/* The pager's TIOCGWINSZ call witnesses actual SIGWINCH dispatch. */
int ioctl(int fd, unsigned long request, ...)
{
    va_list arguments;
    va_start(arguments, request);
    void *argument = va_arg(arguments, void *);
    va_end(arguments);
    int (*real_ioctl)(int, unsigned long, ...) = dlsym(RTLD_NEXT, "ioctl");
    if (!real_ioctl) _exit(125);
    int result = real_ioctl(fd, request, argument);
    if (request == TIOCGWINSZ) {
        const char *gate = getenv("RLESS_RESIZE_GATE");
        const char *hit = getenv("RLESS_RESIZE_HIT");
        if (gate && hit && access(gate, F_OK) == 0) {
            int output = open(hit, O_WRONLY | O_CREAT | O_APPEND | O_CLOEXEC, 0600);
            if (output >= 0) {
                (void)write(output, "hit\n", 4);
                (void)close(output);
            }
        }
    }
    return result;
}
