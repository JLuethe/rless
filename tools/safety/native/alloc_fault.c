/* Test-only, separately linked allocation boundary counterpart.
 * Link with --wrap=calloc and --wrap=strdup. Never preload this object or link it into releases.
 * Every non-selected call forwards to __real_calloc: this preserves the active
 * sanitizer interceptor. No __libc_calloc/dlsym/bootstrap allocator is used.
 *
 * The companion native_alloc.py must bind the exact ELF, symbol sizes and
 * frame-pointer prologues before supplying a plan. Reserve deliberately targets
 * ecalloc called by xbuf_add_byte AFTER size calculation, not the synthetic
 * pre-growth hook. A fatal OOM exit does not prove application allocation cleanup.
 */
#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <unistd.h>

#if !defined(__x86_64__) || !defined(__linux__)
#error "native allocation counterpart supports verified Linux x86_64 only"
#endif

extern void *__real_calloc(size_t, size_t);
extern char *__real_strdup(const char *);
extern void *ecalloc(size_t, size_t);
struct xbuffer;
extern void xbuf_add_byte(struct xbuffer *, unsigned char);

static unsigned active_site;
static uint64_t wanted, attempts;
static uintptr_t ecalloc_size, xbuf_size;
static int trace_fd = -1;
static int parser_fd = -1;
static int parser_fired;

static uint64_t positive(const char *text)
{
    uint64_t result = 0;
    if (!text || !*text) _exit(123);
    for (; *text; ++text) {
        if (*text < '0' || *text > '9') _exit(123);
        unsigned digit = (unsigned)(*text - '0');
        if (result > (UINT64_MAX - digit) / 10) _exit(123);
        result = result * 10 + digit;
    }
    if (!result) _exit(123);
    return result;
}

static int checked_trace(const char *text)
{
    uint64_t descriptor = positive(text);
    if (descriptor > INT32_MAX) _exit(123);
    int fd = (int)descriptor;
    struct stat status;
    int flags = fcntl(fd, F_GETFD);
    if (flags < 0 || fstat(fd, &status) || !S_ISFIFO(status.st_mode) ||
        fcntl(fd, F_SETFD, flags | FD_CLOEXEC)) _exit(123);
    return fd;
}

static void emit(int fd, unsigned site, uint64_t ordinal)
{
    unsigned char bytes[16] = {'R', 'N', 'T', '1', (unsigned char)site, 0, 0, 0};
    for (unsigned i = 0; i < 8; ++i) bytes[8+i] = (unsigned char)(ordinal >> (8*i));
    ssize_t written;
    do {
        written = syscall(SYS_write, fd, bytes, sizeof bytes);
    } while (written < 0 && errno == EINTR);
    if (written != (ssize_t)sizeof bytes) _exit(124);
}

__attribute__((constructor)) static void configure(void)
{
    const char *site = getenv("RLESS_NATIVE_ALLOC_SITE");
    const char *parser = getenv("RLESS_NATIVE_PARSER_FD");
    if (parser) {
        if (site) _exit(123);
        parser_fd = checked_trace(parser);
        if (unsetenv("RLESS_NATIVE_PARSER_FD")) _exit(123);
        return;
    }
    if (!site) return;
    unsigned selected = !strcmp(site, "alloc") ? 1 : !strcmp(site, "reserve") ? 12 : 0;
    if (!selected) _exit(123);
    wanted = positive(getenv("RLESS_NATIVE_ALLOC_ORDINAL"));
    trace_fd = checked_trace(getenv("RLESS_NATIVE_ALLOC_FD"));
    ecalloc_size = (uintptr_t)positive(getenv("RLESS_NATIVE_ALLOC_ECALLOC_SIZE"));
    xbuf_size = (uintptr_t)positive(getenv("RLESS_NATIVE_ALLOC_XBUF_SIZE"));
    if (ecalloc_size > 1048576 || xbuf_size > 1048576) _exit(123);
    /* Helpers must not inherit a plan naming a descriptor closed during exec. */
    if (unsetenv("RLESS_NATIVE_ALLOC_SITE") || unsetenv("RLESS_NATIVE_ALLOC_ORDINAL") ||
        unsetenv("RLESS_NATIVE_ALLOC_FD") || unsetenv("RLESS_NATIVE_ALLOC_ECALLOC_SIZE") ||
        unsetenv("RLESS_NATIVE_ALLOC_XBUF_SIZE")) _exit(123);
    active_site = selected;
}

static int within(uintptr_t address, uintptr_t start, uintptr_t size)
{
    return address >= start && address - start < size;
}

/* GCC warns correctly that return_address(1) is unsafe in arbitrary callers.
 * We only evaluate it AFTER identifying ecalloc, whose actual frame-pointer
 * prologue native_alloc.py verifies in this exact linked ELF. Both the wrapper
 * and Rust counterpart must preserve frame pointers and avoid sibling calls.
 */
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wframe-address"
__attribute__((noinline)) void *__wrap_calloc(size_t count, size_t size)
{
    if (!active_site) return __real_calloc(count, size);
    uintptr_t caller = (uintptr_t)__builtin_extract_return_addr(__builtin_return_address(0));
    if (!within(caller, (uintptr_t)&ecalloc, ecalloc_size)) return __real_calloc(count, size);
    if (active_site == 12) {
        uintptr_t owner = (uintptr_t)__builtin_extract_return_addr(__builtin_return_address(1));
        if (!within(owner, (uintptr_t)&xbuf_add_byte, xbuf_size)) return __real_calloc(count, size);
    }
    if (attempts == UINT64_MAX) _exit(123);
    if (++attempts != wanted) return __real_calloc(count, size);
    emit(trace_fd, active_site, attempts);
    errno = ENOMEM;
    return NULL;
}
#pragma GCC diagnostic pop

/* The filename is deliberately unique to the pre-init parser regression. Every
 * other strdup, including runtime/sanitizer startup, uses the active interceptor.
 * No global string allocation counter or allocator implementation is replaced.
 */
__attribute__((noinline)) char *__wrap_strdup(const char *text)
{
    if (parser_fd >= 0 && !parser_fired && text &&
        !strcmp(text, "rless-early-config-failure")) {
        parser_fired = 1;
        emit(parser_fd, 6, 1);
        errno = ENOMEM;
        return NULL;
    }
    return __real_strdup(text);
}
