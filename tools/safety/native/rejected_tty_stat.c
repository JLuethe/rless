/* Make bad_file observe a regular file while the pinned open sees a tty. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <unistd.h>

static int traced;

int stat(const char *path, struct stat *result) {
    int (*next)(const char *, struct stat *) = dlsym(RTLD_NEXT, "stat");
    if (!next) _exit(125);
    int status = next(path, result);
    if (status == 0 && strcmp(path, "/dev/tty") == 0 &&
        getenv("RLESS_REJECTED_TTY_STAT_TRACE")) {
        if (!traced++) {
            const char *trace = getenv("RLESS_REJECTED_TTY_STAT_TRACE");
            int fd = syscall(SYS_openat, AT_FDCWD, trace,
                             O_WRONLY | O_APPEND | O_CLOEXEC, 0);
            if (fd < 0 || syscall(SYS_write, fd, "regular-stat\n", 13) != 13)
                _exit(124);
            syscall(SYS_close, fd);
        }
        result->st_mode = (result->st_mode & ~S_IFMT) | S_IFREG;
    }
    return status;
}
