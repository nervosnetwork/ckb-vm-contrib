#include <stdio.h>
#include <string.h>

#include "fips202.h"

int test_shake_128_ax1000000() {
    FIPS202_UINT8 hash[32];
    FIPS202_UINT8 want[32] = {0x9d, 0x22, 0x2c, 0x79, 0xc4, 0xff, 0x9d, 0x09,
                              0x2c, 0xf6, 0xca, 0x86, 0x14, 0x3a, 0xa4, 0x11,
                              0xe3, 0x69, 0x97, 0x38, 0x08, 0xef, 0x97, 0x09,
                              0x32, 0x55, 0x82, 0x6c, 0x55, 0x72, 0xef, 0x58};
    FIPS202_UINT64 s_inc[26];
    const char* message = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    shake128_inc_init(s_inc);
    for (int i = 0; i < 25000; i++) {
        shake128_inc_absorb(s_inc, (const FIPS202_UINT8*)message,
                            strlen(message));
    }
    shake128_inc_finalize(s_inc);
    shake128_inc_squeeze(hash, sizeof(hash), s_inc);
    return __builtin_memcmp(hash, want, 32);
}

int test_shake_128_chain() {
    FIPS202_UINT8 m[32] = {0};
    FIPS202_UINT8 want[32] = {0xa0, 0x65, 0x00, 0xf1, 0xf0, 0xd8, 0xa6, 0x38,
                              0xb4, 0xbb, 0x41, 0x2f, 0xed, 0x1e, 0x71, 0xf1,
                              0x9f, 0x68, 0x0b, 0xc0, 0xf8, 0xea, 0x21, 0x3e,
                              0x34, 0x17, 0x06, 0x46, 0x4f, 0x89, 0xa5, 0x9a};
    for (int i = 0; i < 25000; i++) {
        FIPS202_UINT8 hash[32];
        shake128(hash, sizeof(hash), m, sizeof(m));
        memcpy(m, hash, sizeof(m));
    }
    return __builtin_memcmp(m, want, 32);
}

int test_shake_128_empty() {
    FIPS202_UINT8 hash[32];
    FIPS202_UINT8 want[32] = {0x7f, 0x9c, 0x2b, 0xa4, 0xe8, 0x8f, 0x82, 0x7d,
                              0x61, 0x60, 0x45, 0x50, 0x76, 0x05, 0x85, 0x3e,
                              0xd7, 0x3b, 0x80, 0x93, 0xf6, 0xef, 0xbc, 0x88,
                              0xeb, 0x1a, 0x6e, 0xac, 0xfa, 0x66, 0xef, 0x26};
    shake128(hash, sizeof(hash), (const FIPS202_UINT8*)"", 0);
    return __builtin_memcmp(hash, want, 32);
}

int test_shake_128_hello_world() {
    const char* msg = "Hello, World!";
    FIPS202_UINT8 hash[32];
    FIPS202_UINT8 want[32] = {0x2b, 0xf5, 0xe6, 0xde, 0xe6, 0x07, 0x9f, 0xad,
                              0x60, 0x4f, 0x57, 0x31, 0x94, 0xba, 0x84, 0x26,
                              0xbd, 0x4d, 0x30, 0xeb, 0x13, 0xe8, 0xba, 0x2e,
                              0xda, 0xe7, 0x0e, 0x52, 0x9b, 0x57, 0x0c, 0xbd};
    shake128(hash, sizeof(hash), (const FIPS202_UINT8*)msg, strlen(msg));
    return __builtin_memcmp(hash, want, 32);
}

int test_shake_256_ax1000000() {
    FIPS202_UINT8 hash[32];
    FIPS202_UINT8 want[32] = {0x35, 0x78, 0xa7, 0xa4, 0xca, 0x91, 0x37, 0x56,
                              0x9c, 0xdf, 0x76, 0xed, 0x61, 0x7d, 0x31, 0xbb,
                              0x99, 0x4f, 0xca, 0x9c, 0x1b, 0xbf, 0x8b, 0x18,
                              0x40, 0x13, 0xde, 0x82, 0x34, 0xdf, 0xd1, 0x3a};
    FIPS202_UINT64 s_inc[26];
    const char* message = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    shake256_inc_init(s_inc);
    for (int i = 0; i < 25000; i++) {
        shake256_inc_absorb(s_inc, (const FIPS202_UINT8*)message,
                            strlen(message));
    }
    shake256_inc_finalize(s_inc);
    shake256_inc_squeeze(hash, sizeof(hash), s_inc);
    return __builtin_memcmp(hash, want, 32);
}

int test_shake_256_chain() {
    FIPS202_UINT8 m[32] = {0};
    FIPS202_UINT8 want[32] = {0x41, 0xa6, 0x43, 0xad, 0xe0, 0x13, 0xc0, 0xe9,
                              0x9b, 0x31, 0x48, 0x42, 0x40, 0xc3, 0xd1, 0x15,
                              0x94, 0x5d, 0x0e, 0xae, 0xed, 0xda, 0xc9, 0x2f,
                              0xfe, 0xf0, 0x47, 0xf2, 0xea, 0x87, 0x32, 0x3f};
    for (int i = 0; i < 25000; i++) {
        FIPS202_UINT8 hash[32];
        shake256(hash, sizeof(hash), m, sizeof(m));
        memcpy(m, hash, sizeof(m));
    }
    return __builtin_memcmp(m, want, 32);
}

int test_shake_256_empty() {
    FIPS202_UINT8 hash[32];
    FIPS202_UINT8 want[32] = {0x46, 0xb9, 0xdd, 0x2b, 0x0b, 0xa8, 0x8d, 0x13,
                              0x23, 0x3b, 0x3f, 0xeb, 0x74, 0x3e, 0xeb, 0x24,
                              0x3f, 0xcd, 0x52, 0xea, 0x62, 0xb8, 0x1b, 0x82,
                              0xb5, 0x0c, 0x27, 0x64, 0x6e, 0xd5, 0x76, 0x2f};
    shake256(hash, sizeof(hash), (const FIPS202_UINT8*)"", 0);
    return __builtin_memcmp(hash, want, 32);
}

int test_shake_256_hello_world() {
    const char* msg = "Hello, World!";
    FIPS202_UINT8 hash[32];
    FIPS202_UINT8 want[32] = {0xb3, 0xbe, 0x97, 0xbf, 0xd9, 0x78, 0x83, 0x3a,
                              0x65, 0x58, 0x8c, 0xea, 0xe8, 0xa3, 0x4c, 0xf5,
                              0x9e, 0x95, 0x58, 0x5a, 0xf6, 0x20, 0x63, 0xe6,
                              0xb8, 0x9d, 0x07, 0x89, 0xf3, 0x72, 0x42, 0x4e};
    shake256(hash, sizeof(hash), (const FIPS202_UINT8*)msg, strlen(msg));
    return __builtin_memcmp(hash, want, 32);
}

int main() {
    if (test_shake_128_ax1000000() != 0) {
        return 1;
    }
    if (test_shake_128_chain() != 0) {
        return 1;
    }
    if (test_shake_128_empty() != 0) {
        return 1;
    }
    if (test_shake_128_hello_world() != 0) {
        return 1;
    }
    if (test_shake_256_ax1000000() != 0) {
        return 1;
    }
    if (test_shake_256_chain() != 0) {
        return 1;
    }
    if (test_shake_256_empty() != 0) {
        return 1;
    }
    if (test_shake_256_hello_world() != 0) {
        return 1;
    }
    return 0;
}
