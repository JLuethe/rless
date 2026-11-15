#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <unistd.h>

/* Hold the shell supervisor after setpgid but before its readiness packet. */
int setpgid(pid_t pid, pid_t pgid)
{
    static int (*real_setpgid)(pid_t, pid_t);
    if (real_setpgid == NULL)
        real_setpgid = dlsym(RTLD_NEXT, "setpgid");
    int result = real_setpgid(pid, pgid);
    const char *marker = getenv("RLESS_SHELL_SETUP_MARKER");
    const char *release = getenv("RLESS_SHELL_SETUP_RELEASE");
    if (result == 0 && pid == 0 && pgid == 0 && marker && release) {
        FILE *file = fopen(marker, "w");
        if (file != NULL) {
            fprintf(file, "%ld\n", (long)getpid());
            fclose(file);
        }
        while (access(release, F_OK) != 0)
            usleep(10000);
    }
    return result;
}
