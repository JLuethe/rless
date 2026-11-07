#define _GNU_SOURCE
#include <dlfcn.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

ssize_t read(int fd, void *buffer, size_t length)
{
    static int fired;
    ssize_t (*real_read)(int, void *, size_t) = dlsym(RTLD_NEXT, "read");
    ssize_t result = real_read(fd, buffer, length);
    const char *mode = getenv("RLESS_TTY_COMMAND_MODE");
    int resize = mode != NULL && strcmp(mode, "resize") == 0;
    int mark_resize = mode != NULL && strcmp(mode, "mark_resize") == 0;
    int mark_user = mode != NULL && strcmp(mode, "mark_user") == 0;
    int pipe_prompt = mode != NULL && strcmp(mode, "pipe_prompt") == 0;
    int mark_prompt = mode != NULL && strcmp(mode, "mark_prompt") == 0;
    int number = mark_user ? SIGUSR1 : resize || mark_resize ? SIGWINCH : SIGTERM;
    char target = resize ? 'q' : pipe_prompt ? '1' : mark_prompt || mark_resize || mark_user ? 'a' : '\n';
    if (!fired && result == 1 && ((char *) buffer)[0] == target && isatty(fd)) {
        fired = 1;
        const char *text = getenv("RLESS_TTY_COMMAND_TRACE_FD");
        if (text != NULL) {
            int trace = atoi(text);
            sigset_t blocked;
            if (trace >= 0 && sigprocmask(SIG_SETMASK, NULL, &blocked) == 0)
                (void) write(trace,
                             sigismember(&blocked, number) == 1 ? "hit-blocked\n" : "hit-unblocked\n",
                             sigismember(&blocked, number) == 1 ? 12 : 14);
        }
        (void) kill(getpid(), number);
    }
    return result;
}
