#define _GNU_SOURCE
#include <stddef.h>
#include <stdio.h>
#include <regex.h>
#include <signal.h>
#include <termios.h>
#include <sys/stat.h>
#include <curses.h>
#include <assert.h>
#include <stdint.h>
#include <string.h>

static void fixed_register_probe(void) {
    regex_t pattern;
    memset(&pattern, 0, sizeof pattern);
    re_set_syntax(RE_SYNTAX_POSIX_EXTENDED);
    const char *expression = "((a)(b))\\1";
    assert(re_compile_pattern(expression, strlen(expression), &pattern) == NULL);
    struct guarded_register { uint32_t before; regoff_t offset; uint32_t after; };
    struct guarded_register start = { 0x12345678, -1, 0x87654321 };
    struct guarded_register end = start;
    struct re_registers registers = { 1, &start.offset, &end.offset };
    pattern.regs_allocated = REGS_FIXED;
    for (int i = 0; i < 256; ++i) {
        assert(re_search(&pattern, "zabab", 5, 0, 5, &registers) == 1);
        assert(start.offset == 1 && end.offset == 5);
        assert(registers.num_regs == 1 && registers.start == &start.offset && registers.end == &end.offset);
        assert(start.before == 0x12345678 && start.after == 0x87654321);
        assert(end.before == 0x12345678 && end.after == 0x87654321);
    }
    regfree(&pattern);
}

int main(void) {
    fixed_register_probe();
    regex_t layout;
    memset(&layout, 0, sizeof layout);
    layout.regs_allocated = REGS_FIXED;
    size_t flag_offset = 0;
    while (flag_offset < sizeof layout && ((unsigned char *)&layout)[flag_offset] == 0) ++flag_offset;
    assert(flag_offset < sizeof layout);
    printf("{\"glibc_headers\":\"%d.%d\",", __GLIBC__, __GLIBC_MINOR__);
    printf("\"ncurses_headers\":\"%s\",", NCURSES_VERSION);
    printf("\"pointer_bytes\":%zu,\"off_t_bytes\":%zu,", sizeof(void *), sizeof(off_t));
    printf("\"regex_bytes\":%zu,\"regex_alignment\":%zu,", sizeof(regex_t), _Alignof(regex_t));
    printf("\"regex_nsub_offset\":%zu,\"regmatch_bytes\":%zu,", offsetof(regex_t, re_nsub), sizeof(regmatch_t));
    printf("\"regs_fixed\":%d,\"regoff_bytes\":%zu,\"registers_bytes\":%zu,", REGS_FIXED, sizeof(regoff_t), sizeof(struct re_registers));
    printf("\"registers_start_offset\":%zu,\"registers_end_offset\":%zu,", offsetof(struct re_registers, start), offsetof(struct re_registers, end));
    printf("\"regs_flag_byte_offset\":%zu,\"regs_fixed_mask\":%u,\"fixed_register_probe\":true,", flag_offset, ((unsigned char *)&layout)[flag_offset]);
    printf("\"sigaction_bytes\":%zu,\"sigset_bytes\":%zu,", sizeof(struct sigaction), sizeof(sigset_t));
    printf("\"termios_bytes\":%zu,\"stat_bytes\":%zu,", sizeof(struct termios), sizeof(struct stat));
    printf("\"sigint\":%d,\"sigwinch\":%d,\"sigpipe\":%d}\n", SIGINT, SIGWINCH, SIGPIPE);
    return 0;
}
