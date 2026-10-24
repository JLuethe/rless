/* Feasibility probe only: target GNU regex behind an opaque byte-slice ABI. */
#define _GNU_SOURCE
#include <regex.h>
#include <stdlib.h>
#include <string.h>
int rless_probe_match(const unsigned char *pattern, size_t plen,
                      const unsigned char *text, size_t len, int *end) {
    struct re_pattern_buffer expression;
    struct re_registers registers;
    memset(&expression, 0, sizeof expression);
    memset(&registers, 0, sizeof registers);
    re_set_syntax(RE_SYNTAX_POSIX_EXTENDED);
    if (re_compile_pattern((const char *)pattern, plen, &expression)) return -2;
    int position = re_search(&expression, (const char *)text, len, 0, len, &registers);
    if (position >= 0) *end = registers.end[0];
    free(registers.start);
    free(registers.end);
    regfree(&expression);
    return position;
}
