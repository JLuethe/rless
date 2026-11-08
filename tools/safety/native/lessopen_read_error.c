#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdatomic.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

static atomic_int fired;

int close(int fd)
{
    static int (*original)(int);
    if (!original)
        original = dlsym(RTLD_NEXT, "close");
    const char *ready = getenv("RLESS_LESSOPEN_READY");
    const char *hit = getenv("RLESS_LESSOPEN_READ_HIT");
    if (ready && hit && getenv("RLESS_LESSOPEN_CLOSE_ERROR") &&
        access(ready, F_OK) == 0 && atomic_load(&fired) == 0) {
        char executable[256];
        ssize_t length = readlink("/proc/self/exe", executable, sizeof(executable) - 1);
        if (length >= 0) {
            executable[length] = '\0';
            const char *name = strrchr(executable, '/');
            struct stat st;
            if (name && strcmp(name + 1, "rless") == 0 &&
                fstat(fd, &st) == 0 && S_ISFIFO(st.st_mode) &&
                atomic_exchange(&fired, 1) == 0) {
                int marker = open(hit, O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC, 0600);
                if (marker >= 0) {
                    (void)write(marker, "hit\n", 4);
                    (void)original(marker);
                }
                (void)original(fd);
                errno = EIO;
                return -1;
            }
        }
    }
    return original(fd);
}

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
