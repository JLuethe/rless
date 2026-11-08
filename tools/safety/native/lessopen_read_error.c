#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdatomic.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <unistd.h>

static atomic_int fired;

ssize_t read(int fd, void *buffer, size_t size)
{
    static ssize_t (*original)(int, void *, size_t);
    if (!original)
        original = dlsym(RTLD_NEXT, "read");
    const char *ready = getenv("RLESS_LESSOPEN_READY");
    const char *hit = getenv("RLESS_LESSOPEN_READ_HIT");
    const char *length = getenv("RLESS_LESSOPEN_READ_LENGTH");
    if (ready && hit && length && size == (size_t)strtoul(length, NULL, 10) &&
        access(ready, F_OK) == 0 && atomic_load(&fired) == 0) {
        struct stat st;
        if (fstat(fd, &st) == 0 && S_ISFIFO(st.st_mode) &&
            atomic_exchange(&fired, 1) == 0) {
            int marker = open(hit, O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC, 0600);
            if (marker >= 0) {
                (void)write(marker, "hit\n", 4);
                (void)close(marker);
            }
            errno = EIO;
            return -1;
        }
    }
    return original(fd, buffer, size);
}
