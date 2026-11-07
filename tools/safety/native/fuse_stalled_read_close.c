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
static const char *contents;
static int flush_count;
static int stall_ordinal;
static int stall_read;
static int stall_open;

static void release_close(int number) { (void)number; released = 1; }

static void emit_hit_and_wait(void)
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

static int probe_getattr(const char *path, struct stat *out, struct fuse_file_info *info)
{
    (void)info;
    memset(out, 0, sizeof(*out));
    if (strcmp(path, "/") == 0) {
        out->st_mode = S_IFDIR | 0700;
        out->st_nlink = 2;
    } else if (strcmp(path, "/tags") == 0) {
        out->st_mode = S_IFREG | 0600;
        out->st_nlink = 1;
        out->st_size = (off_t)strlen(contents);
    } else return -ENOENT;
    out->st_uid = getuid();
    out->st_gid = getgid();
    return 0;
}

static int probe_open(const char *path, struct fuse_file_info *info)
{
    if (strcmp(path, "/tags") != 0) return -ENOENT;
    if (stall_read) info->direct_io = 1;
    if (stall_open) emit_hit_and_wait();
    return 0;
}

static int probe_read(const char *path, char *buf, size_t size, off_t offset,
                      struct fuse_file_info *info)
{
    (void)info;
    if (strcmp(path, "/tags") != 0) return -ENOENT;
    if (stall_read) {
        int fd = open(marker, O_WRONLY | O_CREAT | O_APPEND | O_CLOEXEC, 0600);
        if (fd >= 0) {
            (void)write(fd, "read\n", 5);
            (void)close(fd);
        }
        while (!released) {
            struct timespec delay = {0, 10000000};
            nanosleep(&delay, NULL);
        }
    }
    size_t len = strlen(contents);
    if (offset < 0 || (size_t)offset >= len) return 0;
    if (size > len - (size_t)offset) size = len - (size_t)offset;
    memcpy(buf, contents + offset, size);
    return (int)size;
}

static int probe_flush(const char *path, struct fuse_file_info *info)
{
    (void)info;
    if (strcmp(path, "/tags") != 0) return -ENOENT;
    if (stall_read || stall_open || ++flush_count != stall_ordinal) return 0;
    emit_hit_and_wait();
    return 0;
}

int main(int argc, char **argv)
{
    marker = getenv("RLESS_FUSE_MARKER");
    contents = getenv("RLESS_FUSE_TAGS");
    const char *ordinal = getenv("RLESS_FUSE_STALL_ORDINAL");
    const char *kind = getenv("RLESS_FUSE_STALL_KIND");
    if (!marker || !contents) return 2;
    stall_read = kind && strcmp(kind, "read") == 0;
    stall_open = kind && strcmp(kind, "open") == 0;
    if (!stall_read && !stall_open) {
        if (!ordinal) return 2;
        stall_ordinal = atoi(ordinal);
        if (stall_ordinal < 1 || stall_ordinal > 2) return 2;
    }
    struct sigaction action = {.sa_handler = release_close};
    sigemptyset(&action.sa_mask);
    if (sigaction(SIGUSR1, &action, NULL) != 0) return 2;
    const struct fuse_operations operations = {
        .getattr = probe_getattr,
        .open = probe_open,
        .read = probe_read,
        .flush = probe_flush,
    };
    return fuse_main(argc, argv, &operations, NULL);
}
