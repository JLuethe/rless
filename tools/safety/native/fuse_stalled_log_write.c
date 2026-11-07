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
static const char *stall_mode;

static void on_release(int number) { (void)number; released = 1; }

static void stall(void)
{
    int fd = open(marker, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
    if (fd >= 0) { (void)write(fd, "hit\n", 4); (void)close(fd); }
    while (!released) {
        struct timespec delay = {0, 10000000};
        nanosleep(&delay, NULL);
    }
}

static int log_getattr(const char *path, struct stat *out,
                       struct fuse_file_info *info)
{
    (void)info;
    memset(out, 0, sizeof(*out));
    if (strcmp(path, "/") == 0) {
        out->st_mode = S_IFDIR | 0700;
        out->st_nlink = 2;
    } else if (strcmp(path, "/log") == 0) {
        out->st_mode = S_IFREG | 0600;
        out->st_nlink = 1;
        out->st_size = 0;
    } else {
        return -ENOENT;
    }
    out->st_uid = getuid();
    out->st_gid = getgid();
    return 0;
}

static int log_open(const char *path, struct fuse_file_info *info)
{
    (void)info;
    if (strcmp(path, "/log") != 0) return -ENOENT;
    if (strcmp(stall_mode, "open") == 0) stall();
    return 0;
}

static int log_truncate(const char *path, off_t size,
                        struct fuse_file_info *info)
{
    (void)size; (void)info;
    if (strcmp(path, "/log") != 0) return -ENOENT;
    if (strcmp(stall_mode, "truncate") == 0) stall();
    return 0;
}

static off_t log_lseek(const char *path, off_t offset, int whence,
                       struct fuse_file_info *info)
{
    (void)info;
    if (strcmp(path, "/log") != 0) return -ENOENT;
    return whence == SEEK_SET ? offset : 0;
}

static int log_write(const char *path, const char *buffer, size_t size,
                     off_t offset, struct fuse_file_info *info)
{
    (void)buffer; (void)offset; (void)info;
    if (strcmp(path, "/log") != 0) return -ENOENT;
    if (strcmp(stall_mode, "write") == 0) stall();
    return (int)size;
}

static int log_flush(const char *path, struct fuse_file_info *info)
{
    (void)info;
    if (strcmp(path, "/log") != 0) return -ENOENT;
    if (strcmp(stall_mode, "flush") == 0) stall();
    return 0;
}

int main(int argc, char **argv)
{
    marker = getenv("RLESS_FUSE_MARKER");
    if (!marker) return 2;
    stall_mode = getenv("RLESS_FUSE_STALL");
    if (!stall_mode || (strcmp(stall_mode, "write") != 0
        && strcmp(stall_mode, "flush") != 0
        && strcmp(stall_mode, "open") != 0
        && strcmp(stall_mode, "truncate") != 0)) return 2;
    struct sigaction action = {.sa_handler = on_release};
    sigemptyset(&action.sa_mask);
    if (sigaction(SIGUSR1, &action, NULL) != 0) return 2;
    const struct fuse_operations operations = {
        .getattr = log_getattr,
        .open = log_open,
        .truncate = log_truncate,
        .lseek = log_lseek,
        .write = log_write,
        .flush = log_flush,
    };
    return fuse_main(argc, argv, &operations, NULL);
}
