#include <stdint.h>
#include <string.h>

#include "api.h"
#include "params.h"
#include "sphincsplus_ref_data.h"

#define SPX_MLEN 32

extern uint8_t sm[];
extern uint8_t pk[];
extern uint8_t m[];

int main() {
    uint8_t mout[SPX_BYTES + SPX_MLEN];
    unsigned long long smlen = sizeof(sm) / sizeof(sm[0]);
    unsigned long long mlen;

    if (crypto_sign_open(mout, &mlen, sm, smlen, pk)) {
        return 1;
    }
    if (mlen != SPX_MLEN) {
        return 1;
    }
    if (memcmp(m, mout, SPX_MLEN)) {
        return 1;
    }
    return 0;
}
