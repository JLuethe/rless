#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static int (*real_sigaction)(int, const struct sigaction *, struct sigaction *);
static const int handled[] = {
    SIGINT, SIGTSTP, SIGWINCH, SIGQUIT, SIGTERM, SIGHUP, SIGALRM, SIGUSR1
};
static struct sigaction original[sizeof handled / sizeof handled[0]];
static sigset_t original_mask;
static int trace_fd = -1;
static int armed;
static int fired;
static int fail_signal = SIGWINCH;

static void trace(const char *message)
{
    if (trace_fd >= 0)
        (void) write(trace_fd, message, strlen(message));
}

static void check_restoration(void)
{
    int actions_ok = 1;
    int mask_ok = 1;
    sigset_t current_mask;
    for (size_t index = 0; index < sizeof handled / sizeof handled[0]; index++) {
        struct sigaction current;
        if (real_sigaction(handled[index], NULL, &current) != 0 ||
            current.sa_sigaction != original[index].sa_sigaction ||
            ((current.sa_flags ^ original[index].sa_flags) & SA_SIGINFO) != 0)
            actions_ok = 0;
    }
    if (sigprocmask(SIG_SETMASK, NULL, &current_mask) != 0) {
        mask_ok = 0;
    } else {
        for (int number = 1; number < NSIG; number++) {
            if (sigismember(&current_mask, number) !=
                sigismember(&original_mask, number))
                mask_ok = 0;
        }
    }
    trace(actions_ok ? "actions-restored\n" : "actions-changed\n");
    trace(mask_ok ? "mask-restored\n" : "mask-changed\n");
}

__attribute__((constructor)) static void install_probe(void)
{
    const char *value = getenv("RLESS_SIGNAL_INSTALL_TRACE_FD");
    if (value == NULL)
        return;
    trace_fd = atoi(value);
    value = getenv("RLESS_SIGNAL_INSTALL_FAIL_SIGNAL");
    if (value != NULL)
        fail_signal = atoi(value);
    real_sigaction = dlsym(RTLD_NEXT, "sigaction");
    if (real_sigaction == NULL ||
        sigprocmask(SIG_SETMASK, NULL, &original_mask) != 0)
        return;
    for (size_t index = 0; index < sizeof handled / sizeof handled[0]; index++) {
        if (real_sigaction(handled[index], NULL, &original[index]) != 0)
            return;
    }
    if (getenv("RLESS_SIGNAL_INSTALL_EXPECT_IGNORED") != NULL)
        trace(original[2].sa_handler == SIG_IGN ?
              "inherited-winch-ignored\n" : "inherited-winch-unexpected\n");
    if (atexit(check_restoration) != 0)
        return;
    armed = 1;
}

int sigaction(int number, const struct sigaction *action, struct sigaction *previous)
{
    if (real_sigaction == NULL)
        real_sigaction = dlsym(RTLD_NEXT, "sigaction");
    if (armed && !fired && number == fail_signal && action != NULL) {
        fired = 1;
        trace("hit\n");
        errno = EIO;
        return -1;
    }
    return real_sigaction(number, action, previous);
}
