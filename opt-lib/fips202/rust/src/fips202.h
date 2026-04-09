#ifndef SPX_FIPS202_H
#define SPX_FIPS202_H

#define FIPS202_UINT8 __UINT8_TYPE__
#define FIPS202_UINT32 __UINT32_TYPE__
#define FIPS202_UINT64 __UINT64_TYPE__
#define FIPS202_SIZE_T __SIZE_TYPE__

#define SHAKE128_RATE 168
#define SHAKE256_RATE 136
#define SHA3_256_RATE 136
#define SHA3_512_RATE 72

void shake128_absorb(FIPS202_UINT64* s, const FIPS202_UINT8* input,
                     FIPS202_SIZE_T inlen);
void shake128_squeezeblocks(FIPS202_UINT8* output, FIPS202_SIZE_T nblocks,
                            FIPS202_UINT64* s);

void shake128_inc_init(FIPS202_UINT64* s_inc);
void shake128_inc_absorb(FIPS202_UINT64* s_inc, const FIPS202_UINT8* input,
                         FIPS202_SIZE_T inlen);
void shake128_inc_finalize(FIPS202_UINT64* s_inc);
void shake128_inc_squeeze(FIPS202_UINT8* output, FIPS202_SIZE_T outlen,
                          FIPS202_UINT64* s_inc);

void shake256_absorb(FIPS202_UINT64* s, const FIPS202_UINT8* input,
                     FIPS202_SIZE_T inlen);
void shake256_squeezeblocks(FIPS202_UINT8* output, FIPS202_SIZE_T nblocks,
                            FIPS202_UINT64* s);

void shake256_inc_init(FIPS202_UINT64* s_inc);
void shake256_inc_absorb(FIPS202_UINT64* s_inc, const FIPS202_UINT8* input,
                         FIPS202_SIZE_T inlen);
void shake256_inc_finalize(FIPS202_UINT64* s_inc);
void shake256_inc_squeeze(FIPS202_UINT8* output, FIPS202_SIZE_T outlen,
                          FIPS202_UINT64* s_inc);

void shake128(FIPS202_UINT8* output, FIPS202_SIZE_T outlen,
              const FIPS202_UINT8* input, FIPS202_SIZE_T inlen);

void shake256(FIPS202_UINT8* output, FIPS202_SIZE_T outlen,
              const FIPS202_UINT8* input, FIPS202_SIZE_T inlen);

void sha3_256_inc_init(FIPS202_UINT64* s_inc);
void sha3_256_inc_absorb(FIPS202_UINT64* s_inc, const FIPS202_UINT8* input,
                         FIPS202_SIZE_T inlen);
void sha3_256_inc_finalize(FIPS202_UINT8* output, FIPS202_UINT64* s_inc);

void sha3_256(FIPS202_UINT8* output, const FIPS202_UINT8* input,
              FIPS202_SIZE_T inlen);

void sha3_512_inc_init(FIPS202_UINT64* s_inc);
void sha3_512_inc_absorb(FIPS202_UINT64* s_inc, const FIPS202_UINT8* input,
                         FIPS202_SIZE_T inlen);
void sha3_512_inc_finalize(FIPS202_UINT8* output, FIPS202_UINT64* s_inc);

void sha3_512(FIPS202_UINT8* output, const FIPS202_UINT8* input,
              FIPS202_SIZE_T inlen);

#endif
