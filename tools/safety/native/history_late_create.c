#define FUSE_USE_VERSION 31
#include <fuse3/fuse.h>
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

static atomic_int released;
static atomic_int created;
static char temporary[256];
static const char *marker;
static const char *created_marker;
static const char *unlinked_marker;

static void mark(const char *path)
{
    int fd = open(path, O_WRONLY | O_CREAT | O_TRUNC | O_CLOEXEC, 0600);
    if (fd >= 0) { (void)write(fd, "hit\n", 4); (void)close(fd); }
}

static void release_create(int number) { (void)number; atomic_store(&released, 1); }

static int fs_getattr(const char *path, struct stat *out, struct fuse_file_info *info)
{
    (void)info;
    memset(out, 0, sizeof(*out));
    if (strcmp(path, "/") == 0) {
        out->st_mode = S_IFDIR | 0700;
        out->st_nlink = 2;
    } else if (atomic_load(&created) && strcmp(path, temporary) == 0) {
        out->st_mode = S_IFREG | 0600;
        out->st_nlink = 1;
    } else {
        return -ENOENT;
    }
    out->st_uid = getuid();
    out->st_gid = getgid();
    return 0;
}

static int fs_opendir(const char *path, struct fuse_file_info *info)
{
    (void)info;
    return strcmp(path, "/") == 0 ? 0 : -ENOENT;
}

static int fs_create(const char *path, mode_t mode, struct fuse_file_info *info)
{
    (void)mode; (void)info;
    if (strncmp(path, "/.historyX.", 11) != 0 || strlen(path) >= sizeof temporary)
        return -EOPNOTSUPP;
    strcpy(temporary, path);
    mark(marker);
    while (!atomic_load(&released)) {
        struct timespec delay = {0, 10000000};
        nanosleep(&delay, NULL);
    }
    atomic_store(&created, 1);
    mark(created_marker);
    return 0;
}

static int fs_unlink(const char *path)
{
    if (!atomic_load(&created) || strcmp(path, temporary) != 0) return -ENOENT;
    atomic_store(&created, 0);
    mark(unlinked_marker);
    return 0;
}

static int fs_open(const char *path, struct fuse_file_info *info)
{
    (void)info;
    return atomic_load(&created) && strcmp(path, temporary) == 0 ? 0 : -ENOENT;
}

static int fs_release(const char *path, struct fuse_file_info *info)
{
    (void)path; (void)info;
    return 0;
}

int main(int argc, char **argv)
{
    marker = getenv("RLESS_FUSE_MARKER");
    created_marker = getenv("RLESS_FUSE_CREATED");
    unlinked_marker = getenv("RLESS_FUSE_UNLINKED");
    if (!marker || !created_marker || !unlinked_marker) return 2;
    struct sigaction action = {.sa_handler = release_create};
    sigemptyset(&action.sa_mask);
    if (sigaction(SIGUSR1, &action, NULL) != 0) return 2;
    const struct fuse_operations operations = {
        .getattr = fs_getattr,
        .opendir = fs_opendir,
        .create = fs_create,
        .open = fs_open,
        .release = fs_release,
        .unlink = fs_unlink,
    };
    return fuse_main(argc, argv, &operations, NULL);
}
