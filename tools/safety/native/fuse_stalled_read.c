#define FUSE_USE_VERSION 31
#include <fuse3/fuse.h>
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

static volatile sig_atomic_t released;
static const char *marker;
static const char *stall_kind;

static void release_read(int number)
{
    (void)number;
    released = 1;
}

static void stall(void)
{
    int fd = open(marker, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
    if (fd >= 0) {
        (void)write(fd, "hit\n", 4);
        (void)close(fd);
    }
    while (!released) {
        struct timespec delay = {0, 10000000};
        nanosleep(&delay, NULL);
    }
}

static int stalled_getattr(const char *path, struct stat *out,
                           struct fuse_file_info *info)
{
    (void)info;
    memset(out, 0, sizeof(*out));
    if (strcmp(path, "/") == 0) {
        out->st_mode = S_IFDIR | 0700;
        out->st_nlink = 2;
    } else if (strcmp(path, "/input") == 0) {
        if (strcmp(stall_kind, "stat") == 0)
            stall();
        out->st_mode = S_IFREG | 0600;
        out->st_nlink = 1;
        out->st_size = 1000;
    } else {
        return -ENOENT;
    }
    out->st_uid = getuid();
    out->st_gid = getgid();
    return 0;
}

static int stalled_open(const char *path, struct fuse_file_info *info)
{
    (void)info;
    return strcmp(path, "/input") == 0 ? 0 : -ENOENT;
}

static int stalled_read(const char *path, char *buffer, size_t size, off_t offset,
                         struct fuse_file_info *info)
{
    (void)info;
    if (strcmp(path, "/input") != 0)
        return -ENOENT;
    if (strcmp(stall_kind, "read") == 0)
        stall();
    static const char line[] = "fuse line\n";
    if (offset >= 1000)
        return 0;
    if (size > (size_t)(1000 - offset))
        size = (size_t)(1000 - offset);
    for (size_t i = 0; i < size; ++i)
        buffer[i] = line[(offset + (off_t)i) % (sizeof(line) - 1)];
    return (int)size;
}

int main(int argc, char **argv)
{
    marker = getenv("RLESS_FUSE_MARKER");
    stall_kind = getenv("RLESS_FUSE_STALL_KIND");
    if (marker == NULL || stall_kind == NULL ||
        (strcmp(stall_kind, "read") != 0 && strcmp(stall_kind, "stat") != 0))
        return 2;
    struct sigaction action = {.sa_handler = release_read};
    sigemptyset(&action.sa_mask);
    if (sigaction(SIGUSR1, &action, NULL) != 0)
        return 2;
    const struct fuse_operations operations = {
        .getattr = stalled_getattr,
        .open = stalled_open,
        .read = stalled_read,
    };
    return fuse_main(argc, argv, &operations, NULL);
}
