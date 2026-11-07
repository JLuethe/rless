/* Test-only native short-write counterpart; never packaged with the pager. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <unistd.h>

/* A private nonblocking pipe writer has a different fd but the same kernel
 * pipe inode as stdout. Follow that verified identity when selecting the
 * production write boundary; the trace pipe has a different inode. */
static int output_fd(int fd)
{
    if (fd == STDOUT_FILENO) return 1;
    if (fd < 0) return 0;
    struct stat stdout_info, target_info;
    return fstat(STDOUT_FILENO, &stdout_info) == 0
        && fstat(fd, &target_info) == 0
        && (stdout_info.st_mode & S_IFMT) == (target_info.st_mode & S_IFMT)
        && stdout_info.st_dev == target_info.st_dev
        && stdout_info.st_ino == target_info.st_ino
        && stdout_info.st_rdev == target_info.st_rdev;
}

ssize_t write(int fd, const void *buffer, size_t count)
{
    static unsigned long attempts;
    ssize_t (*real_call)(int, const void *, size_t) = dlsym(RTLD_NEXT, "write");
    if (!real_call) _exit(125);
    const char *trace = getenv("RLESS_NATIVE_SHORT_WRITE_FD");
    const char *plan = getenv("RLESS_NATIVE_SHORT_WRITE_AT");
    unsigned long ordinal = plan ? strtoul(plan, NULL, 10) : 1;
    if (output_fd(fd) && count > 1 && trace && ++attempts == ordinal) {
        unsigned char marker[16] = {
            'R', 'N', 'T', '1', 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0
        };
        for (unsigned i = 0; i < 8; ++i) marker[8+i] = (unsigned char)((unsigned long long)attempts >> (8*i));
        if (real_call(atoi(trace), marker, sizeof marker) != sizeof marker) _exit(124);
        return real_call(fd, buffer, count / 2);
    }
    return real_call(fd, buffer, count);
}
