#include <stdio.h>
#include <string.h>

#include "fips202.h"

int test_shake_256_ax1000000() {
    uint8_t hash[32];
    uint8_t want[32] = {0x35, 0x78, 0xa7, 0xa4, 0xca, 0x91, 0x37, 0x56,
                        0x9c, 0xdf, 0x76, 0xed, 0x61, 0x7d, 0x31, 0xbb,
                        0x99, 0x4f, 0xca, 0x9c, 0x1b, 0xbf, 0x8b, 0x18,
                        0x40, 0x13, 0xde, 0x82, 0x34, 0xdf, 0xd1, 0x3a};
    uint64_t s_inc[26];
    const char* message = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    shake256_inc_init(s_inc);
    for (int i = 0; i < 25000; i++) {
        shake256_inc_absorb(s_inc, (const uint8_t*)message, strlen(message));
    }
    shake256_inc_finalize(s_inc);
    shake256_inc_squeeze(hash, sizeof(hash), s_inc);
    return __builtin_memcmp(hash, want, 32);
}

int test_shake_256_chain() {
    uint8_t m[32] = {0};
    uint8_t want[32] = {0x41, 0xa6, 0x43, 0xad, 0xe0, 0x13, 0xc0, 0xe9,
                        0x9b, 0x31, 0x48, 0x42, 0x40, 0xc3, 0xd1, 0x15,
                        0x94, 0x5d, 0x0e, 0xae, 0xed, 0xda, 0xc9, 0x2f,
                        0xfe, 0xf0, 0x47, 0xf2, 0xea, 0x87, 0x32, 0x3f};
    for (int i = 0; i < 25000; i++) {
        uint8_t hash[32];
        shake256(hash, sizeof(hash), m, sizeof(m));
        memcpy(m, hash, sizeof(m));
    }
    return __builtin_memcmp(m, want, 32);
}

int test_shake_256_empty() {
    uint8_t hash[32];
    uint8_t want[32] = {0x46, 0xb9, 0xdd, 0x2b, 0x0b, 0xa8, 0x8d, 0x13,
                        0x23, 0x3b, 0x3f, 0xeb, 0x74, 0x3e, 0xeb, 0x24,
                        0x3f, 0xcd, 0x52, 0xea, 0x62, 0xb8, 0x1b, 0x82,
                        0xb5, 0x0c, 0x27, 0x64, 0x6e, 0xd5, 0x76, 0x2f};
    shake256(hash, sizeof(hash), (const uint8_t*)"", 0);
    return __builtin_memcmp(hash, want, 32);
}

int test_shake_256_hello_world() {
    const char* msg = "Hello, World!";
    uint8_t hash[32];
    uint8_t want[32] = {0xb3, 0xbe, 0x97, 0xbf, 0xd9, 0x78, 0x83, 0x3a,
                        0x65, 0x58, 0x8c, 0xea, 0xe8, 0xa3, 0x4c, 0xf5,
                        0x9e, 0x95, 0x58, 0x5a, 0xf6, 0x20, 0x63, 0xe6,
                        0xb8, 0x9d, 0x07, 0x89, 0xf3, 0x72, 0x42, 0x4e};
    shake256(hash, sizeof(hash), (const uint8_t*)msg, strlen(msg));
    return __builtin_memcmp(hash, want, 32);
}

int main() {
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
