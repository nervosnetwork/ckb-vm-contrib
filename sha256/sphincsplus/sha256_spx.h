#ifndef SHA256_SPX_H
#define SHA256_SPX_H

#include <stddef.h>
#include <stdint.h>

typedef uint8_t SHA256_BYTE;
typedef uint32_t SHA256_WORD;
typedef uint64_t SHA256_DWORD;

#define SHA256_BLOCK_SIZE 32

typedef struct {
    uint8_t state[40];
    uint8_t buffer[64];
    size_t buflen;
} SHA256_CTX;

void sha256_init(SHA256_CTX *ctx);
void sha256_update(SHA256_CTX *ctx, const SHA256_BYTE data[], SHA256_DWORD len);
void sha256_final(SHA256_CTX *ctx, SHA256_BYTE hash[]);

#endif
