/* Test-only native counterpart: interrupt one actual tcsetattr restore request.
 * Never linked into, installed with or packaged as the pager. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <stdlib.h>
#include <termios.h>
#include <unistd.h>

int tcsetattr(int fd, int action, const struct termios *value)
{
    static int fired;
    int (*real_call)(int, int, const struct termios *) = dlsym(RTLD_NEXT, "tcsetattr");
    if (!real_call) _exit(125);
    if (!fired && (value->c_lflag & ICANON) && getenv("RLESS_NATIVE_CLEANUP_FD")) {
        static const unsigned char marker[16] = {
            'R', 'N', 'T', '1', 11, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0
        };
        fired = 1;
        int trace = atoi(getenv("RLESS_NATIVE_CLEANUP_FD"));
        if (write(trace, marker, sizeof marker) != sizeof marker) _exit(124);
        errno = EINTR;
        return -1;
    }
    return real_call(fd, action, value);
}
