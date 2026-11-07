#define _GNU_SOURCE
#include <dlfcn.h>
#include <signal.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>
#include <time.h>
#include <string.h>

static int armed;
static int fired;
static int (*real_kill)(pid_t, int);
static int (*real_mask)(int, const sigset_t *, sigset_t *);

__attribute__((constructor)) static void initialize(void) {
    real_kill = dlsym(RTLD_NEXT, "kill");
    real_mask = dlsym(RTLD_NEXT, "sigprocmask");
    if (!real_kill || !real_mask) _exit(127);
}

static void mark_and_terminate(int (*real_kill)(pid_t, int), pid_t pid) {
    fired = 1;
    const char *path = getenv("RLESS_STOP_TERM_TRACE");
    if (path) {
        int fd = open(path, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
        if (fd >= 0) { write(fd, "hit\n", 4); close(fd); }
    }
    real_kill(pid, SIGTERM);
    struct timespec pause = { .tv_sec = 0, .tv_nsec = 10000000 };
    nanosleep(&pause, 0);
}

int kill(pid_t pid, int signal_number) {
    // SIGCONT can reenter this interposer from the pager's TERM handler;
    // short-circuit before touching the test's non-atomic transition state.
    if (signal_number == SIGTSTP && pid == getpid() && !fired) {
        const char *phase = getenv("RLESS_STOP_TERM_PHASE");
        if (phase && strcmp(phase, "unmask") == 0) {
            armed = 1;
        } else {
            mark_and_terminate(real_kill, pid);
        }
    }
    return real_kill(pid, signal_number);
}

int sigprocmask(int how, const sigset_t *set, sigset_t *oldset) {
    if (armed && !fired && how == SIG_SETMASK) {
        mark_and_terminate(real_kill, getpid());
    }
    return real_mask(how, set, oldset);
}
