#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <unistd.h>

static pid_t owner;
static int fired;

__attribute__((constructor)) static void identify_parent(void)
{
    owner = getpid();
}

ssize_t write(int fd, const void *buffer, size_t count)
{
    ssize_t (*real_write)(int, const void *, size_t) = dlsym(RTLD_NEXT, "write");
    if (!real_write) _exit(125);
    const char *trace = getenv("RLESS_PIPE_WRITE_EIO_TRACE");
    struct stat status;
    if (trace && !fired && getpid() == owner && fd > STDERR_FILENO
        && fstat(fd, &status) == 0 && S_ISFIFO(status.st_mode)
        && (fcntl(fd, F_GETFL) & O_ACCMODE) == O_WRONLY) {
        fired = 1;
        int marker = open(trace, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
        if (marker >= 0) {
            (void)syscall(SYS_write, marker, "hit\n", 4);
            (void)close(marker);
        }
        errno = EIO;
        return -1;
    }
    return real_write(fd, buffer, count);
}
