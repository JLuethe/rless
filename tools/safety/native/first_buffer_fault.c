/* Test-only linked calloc wrapper, scoped to ch_addbuf's first 8240-byte buffer. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <sys/syscall.h>
#include <unistd.h>

extern void *__real_calloc(size_t, size_t);
static int fired, checking, matches;

static uintptr_t hex(const char *name) {
    const char *p=getenv(name);
    if (!p || !*p) _exit(123);
    char *end;
    unsigned long long value=strtoull(p,&end,16);
    if (*end) _exit(123);
    return (uintptr_t)value;
}

static void trace_hit(uintptr_t offset) {
    const char *path=getenv("RLESS_BUFFER_TRACE");
    if (!path) _exit(123);
    int fd=syscall(SYS_openat,AT_FDCWD,path,O_WRONLY|O_APPEND|O_CLOEXEC,0);
    char line[100];
    int n=snprintf(line,sizeof line,"ch_addbuf calloc(1,8240) caller_offset=%lx\n",
                   (unsigned long)offset);
    if (fd<0 || n<0 || n>=(int)sizeof line || syscall(SYS_write,fd,line,n)!=n) _exit(124);
    syscall(SYS_close,fd);
}

__attribute__((noinline)) void *__wrap_calloc(size_t count, size_t size) {
    if (count==1 && size==8240 && !fired && !checking && getenv("RLESS_BUFFER_TRACE")) {
        checking=1;
        Dl_info info;
        if (dladdr(__builtin_return_address(0),&info) && info.dli_fbase) {
            uintptr_t offset=(uintptr_t)__builtin_return_address(0)-(uintptr_t)info.dli_fbase;
            if (offset>=hex("RLESS_BUFFER_START") && offset<hex("RLESS_BUFFER_END")) {
                const char *skip=getenv("RLESS_BUFFER_SKIP");
                if (matches++ < (skip ? atoi(skip) : 0)) {
                    checking=0;
                    return __real_calloc(count,size);
                }
                fired=1;
                trace_hit(offset);
                errno=ENOMEM;
                checking=0;
                return NULL;
            }
        }
        checking=0;
    }
    return __real_calloc(count,size);
}
