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
static int ioctl_count;
static int read_reported;

static void release_ioctl(int signum) { (void)signum; released = 1; }

static int probe_getattr(const char *path, struct stat *out, struct fuse_file_info *info)
{
    (void)info;
    memset(out, 0, sizeof(*out));
    if (!strcmp(path, "/")) { out->st_mode = S_IFDIR | 0700; out->st_nlink = 2; }
    else if (!strcmp(path, "/input")) {
        out->st_mode = S_IFREG | 0600; out->st_nlink = 1; out->st_size = 1000;
    } else return -ENOENT;
    out->st_uid = getuid(); out->st_gid = getgid();
    return 0;
}

static int probe_open(const char *path, struct fuse_file_info *info)
{ (void)info; return !strcmp(path, "/input") ? 0 : -ENOENT; }

static int probe_read(const char *path, char *buffer, size_t size, off_t offset,
                      struct fuse_file_info *info)
{
    (void)info;
    if (strcmp(path, "/input")) return -ENOENT;
    if (!read_reported) {
        read_reported = 1;
        FILE *file = fopen(marker, "a");
        if (file) { fputs("read\n", file); fclose(file); }
    }
    static const char line[] = "fuse line\n";
    if (offset >= 1000) return 0;
    if (size > (size_t)(1000 - offset)) size = (size_t)(1000 - offset);
    for (size_t i = 0; i < size; i++) buffer[i] = line[(offset + (off_t)i) % (sizeof(line)-1)];
    return (int)size;
}

static int probe_ioctl(const char *path, int cmd, void *arg,
                       struct fuse_file_info *info, unsigned int flags, void *data)
{
    (void)arg; (void)info; (void)flags; (void)data;
    if (strcmp(path, "/input")) return -ENOENT;
    ioctl_count++;
    FILE *file = fopen(marker, "a");
    if (file) { fprintf(file, "ioctl %d %d\n", ioctl_count, cmd); fclose(file); }
    if (ioctl_count == 2) {
        while (!released) {
            struct timespec delay = {0, 10000000};
            nanosleep(&delay, NULL);
        }
    }
    return -ENOTTY;
}

int main(int argc, char **argv)
{
    marker = getenv("RLESS_IOCTL_MARKER");
    if (!marker) return 2;
    struct sigaction action = {.sa_handler = release_ioctl};
    sigemptyset(&action.sa_mask);
    if (sigaction(SIGUSR1, &action, NULL)) return 2;
    const struct fuse_operations operations = {
        .getattr = probe_getattr, .open = probe_open,
        .read = probe_read, .ioctl = probe_ioctl,
    };
    return fuse_main(argc, argv, &operations, NULL);
}
