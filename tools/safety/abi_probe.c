#define _GNU_SOURCE
#include <stddef.h>
#include <stdio.h>
#include <regex.h>
#include <signal.h>
#include <termios.h>
#include <sys/stat.h>
#include <curses.h>

int main(void) {
    printf("{\"glibc_headers\":\"%d.%d\",", __GLIBC__, __GLIBC_MINOR__);
    printf("\"ncurses_headers\":\"%s\",", NCURSES_VERSION);
    printf("\"pointer_bytes\":%zu,\"off_t_bytes\":%zu,", sizeof(void *), sizeof(off_t));
    printf("\"regex_bytes\":%zu,\"regex_alignment\":%zu,", sizeof(regex_t), _Alignof(regex_t));
    printf("\"regex_nsub_offset\":%zu,\"regmatch_bytes\":%zu,", offsetof(regex_t, re_nsub), sizeof(regmatch_t));
    printf("\"sigaction_bytes\":%zu,\"sigset_bytes\":%zu,", sizeof(struct sigaction), sizeof(sigset_t));
    printf("\"termios_bytes\":%zu,\"stat_bytes\":%zu,", sizeof(struct termios), sizeof(struct stat));
    printf("\"sigint\":%d,\"sigwinch\":%d,\"sigpipe\":%d}\n", SIGINT, SIGWINCH, SIGPIPE);
    return 0;
}
