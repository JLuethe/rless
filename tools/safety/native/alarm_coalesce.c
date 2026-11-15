/* Test-only witness: make an external SIGALRM coalesce with ITIMER_REAL. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <poll.h>
#include <signal.h>
#include <stdlib.h>
#include <sys/time.h>
#include <time.h>
#include <unistd.h>

static int fired;

static void coalesce_once(void) {
    struct itimerval timer;
    if (fired || getitimer(ITIMER_REAL, &timer) != 0 || !timerisset(&timer.it_value)) return;
    fired = 1;
    const char *trace = getenv("RLESS_ALARM_TRACE");
    sigset_t block, saved;
    if (sigemptyset(&block) != 0 || sigaddset(&block, SIGALRM) != 0 ||
        sigprocmask(SIG_BLOCK, &block, &saved) != 0) _exit(125);
    if (kill(getpid(), SIGALRM) != 0) _exit(125);
    struct timespec nap = {
        .tv_sec = timer.it_value.tv_sec,
        .tv_nsec = timer.it_value.tv_usec * 1000 + 200000000
    };
    if (nap.tv_nsec >= 1000000000) {
        nap.tv_sec++;
        nap.tv_nsec -= 1000000000;
    }
    while (nanosleep(&nap, &nap) != 0) {
        if (errno != EINTR) _exit(125);
    }
    if (sigprocmask(SIG_SETMASK, &saved, 0) != 0) _exit(125);
    if (trace) {
        int fd = open(trace, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
        if (fd < 0 || write(fd, "hit\n", 4) != 4 || close(fd) != 0) _exit(125);
    }
}

int poll(struct pollfd *fds, nfds_t count, int timeout) {
    int (*real_call)(struct pollfd *, nfds_t, int) = dlsym(RTLD_NEXT, "poll");
    if (!real_call) _exit(125);
    coalesce_once();
    return real_call(fds, count, timeout);
}

int ppoll(struct pollfd *fds, nfds_t count, const struct timespec *timeout,
          const sigset_t *mask) {
    int (*real_call)(struct pollfd *, nfds_t, const struct timespec *, const sigset_t *) =
        dlsym(RTLD_NEXT, "ppoll");
    if (!real_call) _exit(125);
    coalesce_once();
    return real_call(fds, count, timeout, mask);
}
