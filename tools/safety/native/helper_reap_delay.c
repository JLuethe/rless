#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

/* Model a killed clone child that cannot yet be reaped. The old blocking
 * wait remains trapped; a bounded WNOHANG caller observes a live child. */
pid_t waitpid(pid_t pid, int *status, int options)
{
    static pid_t (*real_waitpid)(pid_t, int *, int);
    if ((options & __WCLONE) != 0) {
        const char *trace = getenv("RLESS_REAP_DELAY_TRACE");
        if (trace != NULL) {
            int fd = open(trace, O_WRONLY | O_CREAT | O_APPEND | O_CLOEXEC, 0600);
            if (fd >= 0) {
                char mode = (options & WNOHANG) != 0 ? 'N' : 'B';
                (void)write(fd, &mode, 1);
                (void)close(fd);
            }
            if ((options & WNOHANG) != 0) {
                return 0;
            }
            for (;;) {
                pause();
            }
        }
    }
    if (real_waitpid == NULL) {
        real_waitpid = dlsym(RTLD_NEXT, "waitpid");
    }
    return real_waitpid(pid, status, options);
}
