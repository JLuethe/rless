/* Test-only witness: deliver another stop during terminal restoration. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdlib.h>
#include <termios.h>
#include <unistd.h>

static int fired;

int tcsetattr(int fd, int action, const struct termios *term) {
    int (*real_call)(int, int, const struct termios *) = dlsym(RTLD_NEXT, "tcsetattr");
    if (!real_call) _exit(125);
    int result = real_call(fd, action, term);
    int saved_errno = errno;
    const char *arm = getenv("RLESS_REPEAT_STOP_ARM");
    const char *trace = getenv("RLESS_REPEAT_STOP_TRACE");
    if (result == 0 && !fired && arm && trace && access(arm, F_OK) == 0 &&
        (term->c_lflag & ICANON)) {
        fired = 1;
        int out = open(trace, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
        if (out < 0 || write(out, "hit\n", 4) != 4 || close(out) != 0 ||
            kill(getpid(), SIGTSTP) != 0) _exit(125);
    }
    errno = saved_errno;
    return result;
}
