#define _GNU_SOURCE
#include <dlfcn.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/socket.h>
#include <sys/uio.h>
#include <unistd.h>

static int fired;
static int *worker_key_read;
static pid_t initial_pid;

__attribute__((constructor)) static void initialize(void)
{
    initial_pid = getpid();
    worker_key_read = mmap(NULL, sizeof *worker_key_read, PROT_READ | PROT_WRITE,
                           MAP_SHARED | MAP_ANONYMOUS, -1, 0);
    if (worker_key_read == MAP_FAILED) _exit(125);
}

static int selected_signal(void)
{
    const char *mode = getenv("RLESS_TTY_COMMAND_MODE");
    if (mode != NULL && strcmp(mode, "mark_user") == 0) return SIGUSR1;
    if (mode != NULL && (strcmp(mode, "resize") == 0 ||
                         strcmp(mode, "mark_resize") == 0 ||
                         strcmp(mode, "pipe_resize") == 0)) return SIGWINCH;
    return SIGTERM;
}

static char selected_key(void)
{
    const char *mode = getenv("RLESS_TTY_COMMAND_MODE");
    if (mode != NULL && strcmp(mode, "resize") == 0) return 'q';
    if (mode != NULL && (strcmp(mode, "pipe_prompt") == 0 ||
                         strcmp(mode, "pipe_resize") == 0)) return '1';
    if (mode != NULL && (strcmp(mode, "mark_prompt") == 0 ||
                         strcmp(mode, "mark_resize") == 0 ||
                         strcmp(mode, "mark_user") == 0)) return 'a';
    return '\n';
}

static void deliver(void)
{
    const char *text = getenv("RLESS_TTY_COMMAND_TRACE_FD");
    int number = selected_signal();
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

ssize_t read(int fd, void *buffer, size_t length)
{
    ssize_t (*real_read)(int, void *, size_t) = dlsym(RTLD_NEXT, "read");
    ssize_t result = real_read(fd, buffer, length);
    if (!fired && result == 1 && ((char *) buffer)[0] == selected_key() && isatty(fd)) {
        fired = 1;
        if (getpid() == initial_pid) deliver();
        else __atomic_store_n(worker_key_read, 1, __ATOMIC_RELEASE);
    }
    return result;
}

ssize_t recvmsg(int fd, struct msghdr *message, int flags)
{
    ssize_t (*next)(int, struct msghdr *, int) = dlsym(RTLD_NEXT, "recvmsg");
    if (!next) _exit(126);
    ssize_t result = next(fd, message, flags);
    if (getpid() == initial_pid && !fired && result > 0 && message &&
        message->msg_iovlen >= 2 && message->msg_iov[1].iov_len >= 1 &&
        __atomic_load_n(worker_key_read, __ATOMIC_ACQUIRE) &&
        *(char *)message->msg_iov[1].iov_base == selected_key()) {
        fired = 1;
        deliver();
    }
    return result;
}
