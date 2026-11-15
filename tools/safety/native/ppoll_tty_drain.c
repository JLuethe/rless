/* Test-only witness: another terminal reader steals a key after ppoll. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <poll.h>
#include <signal.h>
#include <stdlib.h>
#include <sys/syscall.h>
#include <unistd.h>

static int fired;

int ppoll(struct pollfd *fds, nfds_t count, const struct timespec *timeout,
          const sigset_t *mask) {
    int (*real_call)(struct pollfd *, nfds_t, const struct timespec *, const sigset_t *) =
        dlsym(RTLD_NEXT, "ppoll");
    if (!real_call) _exit(125);
    int result = real_call(fds, count, timeout, mask);
    int saved_errno = errno;
    if (fired || result <= 0 || count != 1 || !(fds[0].revents & POLLIN) ||
        !isatty(fds[0].fd)) {
        errno = saved_errno;
        return result;
    }
    int flags = fcntl(fds[0].fd, F_GETFL);
    if (flags < 0 || (flags & O_NONBLOCK)) _exit(125);
    char key = 0;
    if (syscall(SYS_read, fds[0].fd, &key, 1) != 1 || key != 'q') _exit(125);
    fired = 1;
    const char *trace = getenv("RLESS_TTY_DRAIN_TRACE");
    if (!trace) _exit(125);
    int fd = open(trace, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
    if (fd < 0 || write(fd, "hit\n", 4) != 4 || close(fd) != 0) _exit(125);
    errno = saved_errno;
    return result;
}
