#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <signal.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Test-only source of process-directed external SIGALRM observations. The
// production timer is disarmed before the drain, so these cannot be old timer
// events. A finite 1000-event stream keeps the old unbounded drain busy.
int sigtimedwait(const sigset_t *set, siginfo_t *info, const struct timespec *timeout) {
    static _Atomic unsigned calls;
    static int (*real_wait)(const sigset_t *, siginfo_t *, const struct timespec *);
    unsigned index = atomic_fetch_add(&calls, 1);
    const char *trace = getenv("RLESS_REARM_TRACE");
    const char *mode = getenv("RLESS_REARM_MODE");
    int all = mode && strcmp(mode, "all") == 0;
    if (trace && index == 0) {
        FILE *file = fopen(trace, "w");
        if (file) { fputs("hit\n", file); fclose(file); }
    }
    if (!real_wait) real_wait = dlsym(RTLD_NEXT, "sigtimedwait");
    if (!real_wait) { errno = ENOSYS; return -1; }
    // Let the actual blocked test signal be consumed before simulating the
    // continuing hostile sender; otherwise the test would exit on SIGALRM
    // when it restores its original signal mask.
    if (index == 0 && !all) return real_wait(set, info, timeout);
    if (index < 1000 && sigismember(set, SIGALRM) == 1) {
        struct timespec pause = {.tv_sec = 0, .tv_nsec = 1000000};
        nanosleep(&pause, NULL);
        if (info) {
            memset(info, 0, sizeof(*info));
            info->si_signo = SIGALRM;
            info->si_code = SI_USER;
        }
        return SIGALRM;
    }
    return real_wait(set, info, timeout);
}
