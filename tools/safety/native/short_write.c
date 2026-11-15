/* Test-only native short-write counterpart; never packaged with the pager. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdlib.h>
#include <unistd.h>

ssize_t write(int fd, const void *buffer, size_t count)
{
    static unsigned long attempts;
    ssize_t (*real_call)(int, const void *, size_t) = dlsym(RTLD_NEXT, "write");
    if (!real_call) _exit(125);
    const char *trace = getenv("RLESS_NATIVE_SHORT_WRITE_FD");
    const char *plan = getenv("RLESS_NATIVE_SHORT_WRITE_AT");
    unsigned long ordinal = plan ? strtoul(plan, NULL, 10) : 1;
    if (fd == STDOUT_FILENO && count > 1 && trace && ++attempts == ordinal) {
        unsigned char marker[16] = {
            'R', 'N', 'T', '1', 16, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0
        };
        for (unsigned i = 0; i < 8; ++i) marker[8+i] = (unsigned char)((unsigned long long)attempts >> (8*i));
        if (real_call(atoi(trace), marker, sizeof marker) != sizeof marker) _exit(124);
        return real_call(fd, buffer, count / 2);
    }
    return real_call(fd, buffer, count);
}
