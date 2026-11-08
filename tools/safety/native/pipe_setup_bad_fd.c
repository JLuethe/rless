#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <unistd.h>

static pid_t pager_pid;
static int fired;

static void mark_hit(void)
{
    const char *trace = getenv("RLESS_PIPE_SETUP_BAD_FD_TRACE");
    if (!trace) return;
    fired = 1;
    int marker = open(trace, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
    if (marker >= 0) {
        (void)syscall(SYS_write, marker, "hit\n", 4);
        (void)close(marker);
    }
}

__attribute__((constructor)) static void identify_pager(void)
{
    pager_pid = getpid();
}

FILE *fdopen(int fd, const char *mode)
{
    FILE *(*real_fdopen)(int, const char *) = dlsym(RTLD_NEXT, "fdopen");
    if (!real_fdopen) _exit(125);
    FILE *stream = real_fdopen(fd, mode);
    struct stat status;
    const char *injection = getenv("RLESS_PIPE_SETUP_MODE");
    if (stream && injection && strcmp(injection, "bad_fd") == 0
        && !fired && getpid() == pager_pid
        && strcmp(mode, "w") == 0 && fstat(fd, &status) == 0
        && S_ISFIFO(status.st_mode)) {
        mark_hit();
        // Model another owner closing the stream descriptor immediately after
        // fdopen succeeds. pipe_data's F_GETFL must then fail with EBADF.
        (void)close(fd);
    }
    return stream;
}

int sigaction(int signal_number, const struct sigaction *action,
              struct sigaction *previous)
{
    int (*real_sigaction)(int, const struct sigaction *, struct sigaction *) =
        dlsym(RTLD_NEXT, "sigaction");
    if (!real_sigaction) _exit(125);
    const char *injection = getenv("RLESS_PIPE_SETUP_MODE");
    if (injection && strcmp(injection, "sigpipe_install") == 0
        && !fired && getpid() == pager_pid && signal_number == SIGPIPE
        && action && action->sa_handler == SIG_IGN) {
        mark_hit();
        errno = EIO;
        return -1;
    }
    return real_sigaction(signal_number, action, previous);
}
