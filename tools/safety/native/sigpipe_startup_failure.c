#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static int selected(const char *mode)
{
    const char *configured = getenv("RLESS_SIGPIPE_FAILURE");
    return configured != NULL && strcmp(configured, mode) == 0;
}

static void hit(const char *marker)
{
    const char *text = getenv("RLESS_SIGPIPE_TRACE_FD");
    if (text == NULL)
        return;
    char *end = NULL;
    long fd = strtol(text, &end, 10);
    if (end == text || *end != '\0' || fd < 0 || fd > 1024)
        return;
    (void) write((int) fd, marker, strlen(marker));
}

int sigaction(int number, const struct sigaction *action, struct sigaction *previous)
{
    if (number == SIGPIPE && action == NULL && previous != NULL && selected("query")) {
        hit("query\n");
        errno = EIO;
        return -1;
    }
    int (*real_sigaction)(int, const struct sigaction *, struct sigaction *) =
        dlsym(RTLD_NEXT, "sigaction");
    return real_sigaction(number, action, previous);
}

void (*signal(int number, void (*handler)(int)))(int)
{
    static int sigpipe_calls;
    // Rust's runtime makes the first call before main. Inject the pager's
    // later restoration so the oracle reaches the application failure path.
    if (number == SIGPIPE && ++sigpipe_calls == 2 && selected("restore")) {
        hit("restore\n");
        errno = EIO;
        return SIG_ERR;
    }
    void (*(*real_signal)(int, void (*)(int)))(int) = dlsym(RTLD_NEXT, "signal");
    return real_signal(number, handler);
}
