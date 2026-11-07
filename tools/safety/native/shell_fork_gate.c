/* Test-only fork gate: pause the shell child before libc::fork returns. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

pid_t fork(void)
{
    pid_t (*real_call)(void) = dlsym(RTLD_NEXT, "fork");
    if (!real_call) _exit(125);
    pid_t child = real_call();
    if (child != 0) return child;
    const char *directory = getenv("RLESS_TEST_SHELL_FORK_GATE");
    if (!directory) return child;
    /* The pager is the PTY session leader in this test. Its death sends
     * terminal SIGHUP to the foreground shell group; isolate the parent-PID
     * check from that independent terminal behavior. */
    signal(SIGHUP, SIG_IGN);
    char marker[4096], release[4096], pid_text[64];
    if (snprintf(marker, sizeof marker, "%s/child", directory) >= (int)sizeof marker ||
        snprintf(release, sizeof release, "%s/release", directory) >= (int)sizeof release)
        _exit(126);
    int fd = open(marker, O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC, 0600);
    if (fd < 0) _exit(126);
    int count = snprintf(pid_text, sizeof pid_text, "%ld\n", (long)getpid());
    if (count <= 0 || write(fd, pid_text, (size_t)count) != count) _exit(126);
    close(fd);
    while (access(release, F_OK) != 0) usleep(1000);
    return child;
}
