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
#include <sys/mman.h>
#include <sys/socket.h>
#include <unistd.h>

struct shared_fault_state {
    unsigned long long attempts;
    unsigned long long pending_ordinal;
    unsigned pending_site;
};

static struct shared_fault_state *shared;
static int input_fds[1024];
static int output_fds[1024];
static int (*current_putc)(int);

__attribute__((constructor)) static void initialize_shared_state(void)
{
    shared = mmap(NULL, sizeof *shared, PROT_READ | PROT_WRITE,
                  MAP_SHARED | MAP_ANONYMOUS, -1, 0);
    if (shared == MAP_FAILED) _exit(125);
}

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

static int selected_inner(const char *name, unsigned site, int defer_event)
{
    const char *plan = getenv("RLESS_NATIVE_IO_SITE");
    if (!plan || strcmp(plan, name)) return 0;
    const char *text = getenv("RLESS_NATIVE_IO_ORDINAL");
    if (!text || !*text) _exit(123);
    char *end;
    unsigned long long requested = strtoull(text, &end, 10);
    if (*end || !requested) _exit(123);
    unsigned long long attempt = __atomic_add_fetch(&shared->attempts, 1, __ATOMIC_SEQ_CST);
    if (!attempt || attempt != requested) return 0;
    if (defer_event) {
        /* The read worker closes inherited trace FDs before pread. Its
         * shared marker is emitted by the parent after recvmsg succeeds. */
        shared->pending_ordinal = attempt;
        __atomic_store_n(&shared->pending_site, site, __ATOMIC_RELEASE);
    } else {
        event(site, attempt);
    }
    return 1;
}

static int selected(const char *name, unsigned site)
{
    return selected_inner(name, site, 0);
}

ssize_t pread(int fd, void *buffer, size_t count, off_t offset)
{
    ssize_t (*real_call)(int, void *, size_t, off_t) = dlsym(RTLD_NEXT, "pread");
    if (!real_call) _exit(125);
    if (fd == 3) {
        if (selected_inner("read", 2, 1)) { errno = EIO; return -1; }
        if (selected_inner("read-eintr", 14, 1)) { errno = EINTR; return -1; }
        if (selected_inner("read-short", 15, 1) && count > 1) count = 1;
    }
    return real_call(fd, buffer, count, offset);
}

ssize_t recvmsg(int fd, struct msghdr *message, int flags)
{
    ssize_t (*real_call)(int, struct msghdr *, int) = dlsym(RTLD_NEXT, "recvmsg");
    if (!real_call) _exit(125);
    ssize_t result = real_call(fd, message, flags);
    if (result >= 0) {
        unsigned site = __atomic_exchange_n(&shared->pending_site, 0, __ATOMIC_ACQUIRE);
        if (site) event(site, shared->pending_ordinal);
    }
    return result;
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
    int output = fd == STDOUT_FILENO || (fd >= 0 && fd < 1024 && output_fds[fd]);
    if (output && selected("write", 4)) { errno = EIO; return -1; }
    if (output && selected("write-eintr", 4)) { errno = EINTR; return -1; }
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
    int named_input = !strcmp(name, "i") || !strcmp(name, "j");
    int input = named_input;
    int output = 0;
    /* The pager reopens an input through /proc/self/fd after pinning it.
     * Preserve the original input identity across that private descriptor. */
    if (!strncmp(path, "/proc/self/fd/", 14)) {
        char *end;
        long source = strtol(path + 14, &end, 10);
        if (!*end && source >= 0 && source < 1024) {
            if (source == STDIN_FILENO || input_fds[source]) input = 1;
            if (source == STDOUT_FILENO || output_fds[source]) output = 1;
        }
    }
    if (named_input && selected("open", 3)) { errno = EACCES; return -1; }
    int fd = real_call(path, flags, mode);
    if (input && fd >= 0 && fd < 1024) input_fds[fd] = 1;
    if (output && fd >= 0 && fd < 1024) output_fds[fd] = 1;
    return fd;
}

int close(int fd)
{
    int (*real_call)(int) = dlsym(RTLD_NEXT, "close");
    if (!real_call) _exit(125);
    int input = fd >= 0 && fd < 1024 && input_fds[fd];
    if (input) input_fds[fd] = 0;
    if (fd >= 0 && fd < 1024) output_fds[fd] = 0;
    int result = real_call(fd);
    if (input && selected("close", 13)) { errno = EIO; return -1; }
    return result;
}

int socketpair(int domain, int type, int protocol, int sv[2])
{
    int (*real_call)(int, int, int, int[2]) = dlsym(RTLD_NEXT, "socketpair");
    if (!real_call) _exit(125);
    /* The managed shell handoff creates one private seqpacket socketpair
     * before its supervisor exists. Fail that native setup boundary before
     * any child or terminal ownership transfer. */
    if (domain == AF_UNIX && (type & SOCK_SEQPACKET) == SOCK_SEQPACKET &&
        (type & SOCK_CLOEXEC) && !(type & SOCK_NONBLOCK) &&
        selected("handoff", 9)) {
        errno = EAGAIN;
        return -1;
    }
    return real_call(domain, type, protocol, sv);
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
