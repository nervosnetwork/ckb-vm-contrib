#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include "sha256_spx.h"

/* Prevent sha2.c from including its problematic headers */
#define SPX_UTILS_H
#define SPX_SHA2_H

/* Define required macros from params.h */
#define SPX_NAMESPACE(s) s
#define SPX_N 32
#define SPX_SHA512 0
#define SPX_SHA256_BLOCK_BYTES 64
#define SPX_SHA256_OUTPUT_BYTES 32
#define SPX_SHA512_BLOCK_BYTES 128
#define SPX_SHA512_OUTPUT_BYTES 64

typedef struct {
    uint8_t pub_seed[32];
    uint8_t state_seeded[40];
} spx_ctx;

#define SPX_VLA(__t, __x, __s) __t __x[__s]
static inline void u32_to_bytes(unsigned char *out, uint32_t in) {
    out[0] = (unsigned char)(in >> 24);
    out[1] = (unsigned char)(in >> 16);
    out[2] = (unsigned char)(in >> 8);
    out[3] = (unsigned char)in;
}

void sha256_inc_init(uint8_t *state);
void sha256_inc_blocks(uint8_t *state, const uint8_t *in, size_t inblocks);
void sha256_inc_finalize(uint8_t *out, uint8_t *state, const uint8_t *in, size_t inlen);

#include "sphincsplus/ref/sha2.c"

/*
 * Compatibility layer with the old API
 */

void sha256_init(SHA256_CTX *ctx) {
    sha256_inc_init(ctx->state);
    ctx->buflen = 0;
}

void sha256_update(SHA256_CTX *ctx, const SHA256_BYTE data[], SHA256_DWORD len) {
    size_t remaining = len;
    const uint8_t *p = data;

    if (ctx->buflen > 0) {
        size_t needed = 64 - ctx->buflen;
        if (remaining < needed) {
            __builtin_memcpy(ctx->buffer + ctx->buflen, p, remaining);
            ctx->buflen += remaining;
            return;
        }
        __builtin_memcpy(ctx->buffer + ctx->buflen, p, needed);
        sha256_inc_blocks(ctx->state, ctx->buffer, 1);
        p += needed;
        remaining -= needed;
        ctx->buflen = 0;
    }

    if (remaining >= 64) {
        size_t blocks = remaining / 64;
        sha256_inc_blocks(ctx->state, p, blocks);
        p += blocks * 64;
        remaining -= blocks * 64;
    }

    if (remaining > 0) {
        __builtin_memcpy(ctx->buffer, p, remaining);
        ctx->buflen = remaining;
    }
}

void sha256_final(SHA256_CTX *ctx, SHA256_BYTE hash[]) {
    sha256_inc_finalize(hash, ctx->state, ctx->buffer, ctx->buflen);
}
