#include <stdio.h>
#include <stdint.h>
#include <string.h>

static const uint64_t REGEX_CASE_INSENSITIVE = 1;
static const uint64_t REGEX_MULTI_LINE = 2;
static const uint64_t REGEX_DOT_MATCHES_NEW_LINE = 4;
static const uint64_t REGEX_IGNORE_WHITESPACE = 8;
static const uint64_t REGEX_UNICODE = 16;
static const uint64_t REGEX_OCTAL = 32;

extern void* make_state(uint64_t reserved_values);
extern void free_state(void* state);
extern void* alloc_string(size_t s);
extern char* get_string_buf(void* str);
extern size_t get_string_len(void* str);
extern uint64_t make_string(void* state, uint64_t idx, void* str, size_t len);
extern uint64_t make_regex(void* state, uint64_t idx, uint64_t regex_flags, void* str, size_t len);
extern uint64_t make_bool(void* state, uint64_t idx, uint32_t b);
extern uint64_t make_i64(void* state, uint64_t idx, int64_t intval);
extern uint64_t make_f64(void* state, uint64_t idx, double fval);
extern uint64_t rtl_eq(void* state, uint64_t left, uint64_t right);
extern uint64_t rtl_and(void* state, uint64_t left, uint64_t right);
extern uint64_t rtl_or(void* state, uint64_t left, uint64_t right);
extern uint64_t rtl_not(void* state, uint64_t value);
extern uint32_t rtl_type(void* state, uint64_t value);
extern uint32_t rtl_get_bool(void* state, uint64_t value);
extern void rtl_make_permanent(void* state, uint64_t value);
extern void rtl_unmake_permanent(void* state, uint64_t value);
extern uint32_t rtl_is_permanent(void* state, uint64_t value);
extern size_t rtl_clear(void* state);

uint64_t load_string(void* state, uint64_t idx, const char* s) {
    void* s_dest = alloc_string(strlen(s));
    printf("strptr: %p\n", s_dest);
    char* buf = get_string_buf(s_dest);
    memcpy(buf, s, strlen(s));
    return make_string(state, idx, s_dest, strlen(s));
}

uint64_t load_regex(void* state, uint64_t idx, uint64_t regex_flags, const char* s) {
    void* s_dest = alloc_string(strlen(s));
    printf("strptr: %p\n", s_dest);
    char* buf = get_string_buf(s_dest);
    memcpy(buf, s, strlen(s));
    return make_regex(state, idx, regex_flags, s_dest, strlen(s));
}

int main(int argc, char** argv) {
    void* state = make_state(1000);

    uint64_t s1 = load_string(state, 42, "Hello World");
    uint64_t s2 = load_string(state, 0, "Hello World");
    uint64_t s3 = load_string(state, 0, "Hello, World!");
    uint64_t s4 = load_string(state, 0, "Hello World");

    uint64_t r1 = load_regex(state, 0, REGEX_UNICODE, "World$");

    printf("Allocate three strings: %ld, %ld and %ld\n", s1, s2, s3);

    uint64_t eq1 = rtl_eq(state, s1, s2);
    uint64_t eq2 = rtl_eq(state, s2, s3);
    uint64_t eq3 = rtl_eq(state, s1, s3);
    uint64_t eq4 = rtl_eq(state, s1, s4);

    uint64_t eq5 = rtl_eq(state, s1, r1);
    uint64_t eq6 = rtl_eq(state, s2, r1);
    uint64_t eq7 = rtl_eq(state, s3, r1);
    uint64_t eq8 = rtl_eq(state, s4, r1);

    uint64_t iv1 = make_i64(state, 0, 17);
    uint64_t iv2 = make_i64(state, 0, 23);
    uint64_t iv3 = make_i64(state, 0, 17);
    uint64_t iv4 = make_i64(state, 0, 23);

    uint64_t fv1 = make_f64(state, 0, 17.23);
    uint64_t fv2 = make_f64(state, 0, 23.00);
    uint64_t fv3 = make_f64(state, 0, 17.23);
    uint64_t fv4 = make_f64(state, 0, 17.23);

    uint64_t eq9 = rtl_eq(state, iv1, iv2);
    uint64_t eq10 = rtl_eq(state, iv1, iv3);
    uint64_t eq11 = rtl_eq(state, iv1, iv4);
    uint64_t eq12 = rtl_eq(state, iv2, iv4);

    uint64_t eq13 = rtl_eq(state, fv1, fv2);
    uint64_t eq14 = rtl_eq(state, fv1, fv3);
    uint64_t eq15 = rtl_eq(state, fv1, fv4);
    uint64_t eq16 = rtl_eq(state, fv2, fv4);

    uint64_t and_t = rtl_and(state, eq13, eq14);
    uint64_t or_t = rtl_or(state, eq13, eq14);
    uint64_t not_t = rtl_not(state, or_t);

    printf("eq13 and eq14 | %s\n", rtl_get_bool(state, and_t) ? "true" : "false");
    printf("eq13 or eq14 | %s\n", rtl_get_bool(state, or_t) ? "true" : "false");
    printf("!(eq13 or eq14) | %s\n", rtl_get_bool(state, not_t) ? "true" : "false");

    printf("iv1 == iv2 | %s\n", rtl_get_bool(state, eq9) ? "true" : "false");
    printf("iv1 == iv3 | %s\n", rtl_get_bool(state, eq10) ? "true" : "false");
    printf("iv1 == iv4 | %s\n", rtl_get_bool(state, eq11) ? "true" : "false");
    printf("iv2 == iv4 | %s\n", rtl_get_bool(state, eq12) ? "true" : "false");

    printf("fv1 == fv2 | %s\n", rtl_get_bool(state, eq13) ? "true" : "false");
    printf("fv1 == fv3 | %s\n", rtl_get_bool(state, eq14) ? "true" : "false");
    printf("fv1 == fv4 | %s\n", rtl_get_bool(state, eq15) ? "true" : "false");
    printf("fv2 == fv4 | %s\n", rtl_get_bool(state, eq16) ? "true" : "false");

    printf("s1 == s2 | %s\n", rtl_get_bool(state, eq1) ? "true" : "false");
    printf("s2 == s3 | %s\n", rtl_get_bool(state, eq2) ? "true" : "false");
    printf("s1 == s3 | %s\n", rtl_get_bool(state, eq3) ? "true" : "false");
    printf("s1 == s4 | %s\n", rtl_get_bool(state, eq4) ? "true" : "false");

    printf("s1 ~= r1 | %s\n", rtl_get_bool(state, eq5) ? "true" : "false");
    printf("s2 ~= r1 | %s\n", rtl_get_bool(state, eq6) ? "true" : "false");
    printf("s3 ~= r1 | %s\n", rtl_get_bool(state, eq7) ? "true" : "false");
    printf("s4 ~= r1 | %s\n", rtl_get_bool(state, eq8) ? "true" : "false");

    printf("Making string 2 permanent...\n");
    rtl_make_permanent(state, s2);
    printf("String 1 is permanent: %s\n", rtl_is_permanent(state, s1) ? "true" : "false");
    printf("String 2 is permanent: %s\n", rtl_is_permanent(state, s2) ? "true" : "false");
    printf("String 3 is permanent: %s\n", rtl_is_permanent(state, s3) ? "true" : "false");
    printf("String 4 is permanent: %s\n", rtl_is_permanent(state, s4) ? "true" : "false");

    printf("Cleaning environment...\n");
    size_t num_objs_cleaned = rtl_clear(state);
    printf("Cleaned %ld objects\n", num_objs_cleaned);

    free_state(state);
    return 0;
}
