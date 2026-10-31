/* Test-only native boundary counterparts. Never linked into a release pager.
 * Selected calls fail before their native action, except close which really
 * releases the descriptor before reporting EIO (Linux must not retry close).
 * Counters and callback context are confined to the single-threaded test pager.
 */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <link.h>
#include <regex.h>
#include <stdarg.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static unsigned long long attempts;
static int input_fds[1024];
static int (*current_putc)(int);

struct callback_target { uintptr_t offset, address; int found; };

static int main_callback(struct dl_phdr_info *info, size_t size, void *data)
{
    (void)size;
    struct callback_target *target = data;
    if (info->dlpi_name && *info->dlpi_name) return 0;
    if (UINTPTR_MAX - info->dlpi_addr < target->offset) _exit(122);
    for (unsigned i = 0; i < info->dlpi_phnum; ++i) {
        const ElfW(Phdr) *segment = &info->dlpi_phdr[i];
        if (segment->p_type == PT_LOAD && (segment->p_flags & PF_X) &&
            target->offset >= segment->p_vaddr &&
            target->offset - segment->p_vaddr < segment->p_memsz) {
            target->address = info->dlpi_addr + target->offset;
            target->found = 1;
            return 1;
        }
    }
    _exit(122);
}

static int selected_callback(int (*putc_fn)(int))
{
    const char *text = getenv("RLESS_NATIVE_CALLBACK_OFFSET");
    if (!text || !*text || *text < '0' || *text > '9') _exit(122);
    char *end;
    errno = 0;
    unsigned long long offset = strtoull(text, &end, 10);
    if (*end || errno || !offset || offset > UINTPTR_MAX) _exit(122);
    struct callback_target target = {(uintptr_t)offset, 0, 0};
    dl_iterate_phdr(main_callback, &target);
    if (!target.found) _exit(122);
    return (uintptr_t)putc_fn == target.address;
}

static void event(unsigned site, unsigned long long ordinal)
{
    const char *text = getenv("RLESS_NATIVE_IO_FD");
    ssize_t (*real_write)(int, const void *, size_t) = dlsym(RTLD_NEXT, "write");
    if (!text || !real_write) _exit(125);
    unsigned char bytes[16] = {'R', 'N', 'T', '1', (unsigned char)site, 0, 0, 0};
    for (unsigned i = 0; i < 8; ++i) bytes[8+i] = (unsigned char)(ordinal >> (8*i));
    if (real_write(atoi(text), bytes, sizeof bytes) != sizeof bytes) _exit(124);
}

static int selected(const char *name, unsigned site)
{
    const char *plan = getenv("RLESS_NATIVE_IO_SITE");
    if (!plan || strcmp(plan, name)) return 0;
    const char *text = getenv("RLESS_NATIVE_IO_ORDINAL");
    if (!text || !*text) _exit(123);
    char *end;
    unsigned long long requested = strtoull(text, &end, 10);
    if (*end || !requested || attempts == UINT64_MAX) _exit(123);
    if (++attempts != requested) return 0;
    event(site, attempts);
    return 1;
}

ssize_t read(int fd, void *buffer, size_t count)
{
    ssize_t (*real_call)(int, void *, size_t) = dlsym(RTLD_NEXT, "read");
    if (!real_call) _exit(125);
    if (fd == STDIN_FILENO || (fd >= 0 && fd < 1024 && input_fds[fd])) {
        if (selected("read", 2)) { errno = EIO; return -1; }
        if (selected("read-eintr", 14)) { errno = EINTR; return -1; }
        if (selected("read-short", 15) && count > 1) count = 1;
    }
    return real_call(fd, buffer, count);
}

ssize_t write(int fd, const void *buffer, size_t count)
{
    ssize_t (*real_call)(int, const void *, size_t) = dlsym(RTLD_NEXT, "write");
    if (!real_call) _exit(125);
    if (fd == STDOUT_FILENO && selected("write", 4)) { errno = EIO; return -1; }
    if (fd == STDOUT_FILENO && selected("write-eintr", 4)) { errno = EINTR; return -1; }
    return real_call(fd, buffer, count);
}

int open(const char *path, int flags, ...)
{
    int (*real_call)(const char *, int, ...) = dlsym(RTLD_NEXT, "open");
    if (!real_call) _exit(125);
    mode_t mode = 0;
    if ((flags & O_CREAT) || (flags & O_TMPFILE) == O_TMPFILE) {
        va_list args;
        va_start(args, flags);
        mode = (mode_t)va_arg(args, int);
        va_end(args);
    }
    const char *name = strrchr(path, '/');
    name = name ? name + 1 : path;
    int input = !strcmp(name, "i") || !strcmp(name, "j");
    if (input && selected("open", 3)) { errno = EACCES; return -1; }
    int fd = real_call(path, flags, mode);
    if (input && fd >= 0 && fd < 1024) input_fds[fd] = 1;
    return fd;
}

int close(int fd)
{
    int (*real_call)(int) = dlsym(RTLD_NEXT, "close");
    if (!real_call) _exit(125);
    int input = fd >= 0 && fd < 1024 && input_fds[fd];
    if (input) input_fds[fd] = 0;
    int result = real_call(fd);
    if (input && selected("close", 13)) { errno = EIO; return -1; }
    return result;
}

int system(const char *command)
{
    int (*real_call)(const char *) = dlsym(RTLD_NEXT, "system");
    if (!real_call) _exit(125);
    if (command && selected("handoff", 9)) { errno = EAGAIN; return -1; }
    return real_call(command);
}

const char *re_compile_pattern(const char *pattern, size_t length, struct re_pattern_buffer *buffer)
{
    const char *(*real_call)(const char *, size_t, struct re_pattern_buffer *) =
        dlsym(RTLD_NEXT, "re_compile_pattern");
    if (!real_call) _exit(125);
    if (selected("regex", 5)) { errno = EIO; return "Invalid regular expression"; }
    return real_call(pattern, length, buffer);
}

static int native_putc(int character)
{
    if (selected("callback", 10)) { errno = EIO; return -1; }
    return current_putc(character);
}

int tputs(const char *capability, int affected, int (*putc_fn)(int))
{
    int saved_errno = errno;
    int (*real_call)(const char *, int, int (*)(int)) = dlsym(RTLD_NEXT, "tputs");
    if (!real_call) _exit(125);
    const char *plan = getenv("RLESS_NATIVE_IO_SITE");
    if (!plan || strcmp(plan, "callback") || !selected_callback(putc_fn)) {
        errno = saved_errno;
        return real_call(capability, affected, putc_fn);
    }
    /* Nested callback registration is rejected, never silently overwritten. */
    if (current_putc) _exit(122);
    current_putc = putc_fn;
    errno = saved_errno;
    int result = real_call(capability, affected, native_putc);
    current_putc = NULL;
    return result;
}
