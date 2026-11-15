#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

off_t lseek(int fd, off_t offset, int whence)
{
    static off_t (*original)(int, off_t, int);
    if (!original) original = dlsym(RTLD_NEXT, "lseek");
    const char *mode = getenv("RLESS_SEEK_ERROR_PROBE");
    if (mode && ((strcmp(mode, "reset") == 0 && offset == 0 && whence == SEEK_SET) ||
                 (strcmp(mode, "probe") == 0 && offset == 1 && whence == SEEK_SET))) {
        struct stat identity;
        if (fstat(fd, &identity) == 0 && S_ISREG(identity.st_mode)) {
            const char *marker = getenv("RLESS_SEEK_ERROR_MARKER");
            if (marker && access(marker, F_OK) != 0) {
                int output = open(marker, O_WRONLY | O_CREAT | O_APPEND | O_CLOEXEC, 0600);
                if (output >= 0) { (void)write(output, "hit\n", 4); (void)close(output); }
                errno = EIO;
                return -1;
            }
        }
    }
    return original(fd, offset, whence);
}
