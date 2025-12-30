.section .rodata
k:
    .word 0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5
    .word 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174
    .word 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da
    .word 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967
    .word 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85
    .word 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070
    .word 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3
    .word 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2

.text
.globl sha256_transform
sha256_transform:
    addi   sp, sp, -336
    sd	   s0, 328(sp)
    sd	   s1, 320(sp)
    sd	   s2, 312(sp)
    sd	   s3, 304(sp)
    sd	   s4, 296(sp)
    sd	   s5, 288(sp)
    sd	   s6, 280(sp)
    sd	   s7, 272(sp)
    sd	   s8, 264(sp)

    mv	   a2, a1
    addi   a3, sp, 8
    addi   a4, sp, 72
.sha256_transform_loop_1:
    ld	   a5, 0(a2)                       # Load 8 bytes (64-bit)
    rev8   a5, a5                          # Reverse all 8 bytes
    sw	   a5, 4(a3)                       # Store lower 32 bits (originally bytes 4-7)
    srli   a5, a5, 32                      # Shift to get upper 32 bits
    sw	   a5, 0(a3)                       # Store upper 32 bits (originally bytes 0-3)
    addi   a2, a2, 8
    addi   a3, a3, 8
    bne	   a3, a4, .sha256_transform_loop_1

    addi   a2, sp, 72
    addi   a3, sp, 264
    lw     a4, 8(sp)                       # Store m[i-16] as a variable for this and next loop
.sha256_transform_loop_2:
    lw     a5, -8(a2)                      # Do s := SIG1(m[i - 2])
    roriw  s0, a5, 17
    roriw  s1, a5, 19
    xor    s0, s0, s1
    srliw  a5, a5, 10
    xor	   a5, a5, s0
    lw	   a6, -28(a2)                     # Do s += m[i-7]
    add    a5, a5, a6
    add	   a5, a5, a4                      # Do s += m[i-16]
    lwu	   a4, -60(a2)                     # Do s += SIG0(m[i-15])
    roriw  s0, a4, 7
    roriw  s1, a4, 18
    xor	   s0, s0, s1
    srliw  a6, a4, 3
    xor	   a6, a6, s0
    add	   a5, a5, a6
    sw	   a5, 0(a2)                       # Do m[i] = s
    addi   a2, a2, 4
    bne	   a2, a3, .sha256_transform_loop_2

    lw     t0, 80(a0)
    lw     t1, 84(a0)
    lw     t2, 88(a0)
    lw     t3, 92(a0)
    lw     t4, 96(a0)
    lw     t5, 100(a0)
    lw     t6, 104(a0)
    lw     s8, 108(a0)
    mv     s0, t0
    mv     s1, t1
    mv     s2, t2
    mv     s3, t3
    mv     s4, t4
    mv     s5, t5
    mv     s6, t6
    mv     s7, s8
    addi   a2, sp, 8
    la     a3, k
    addi   a4, a3, 256
.sha256_transform_loop_3:
    roriw  a5, s4, 6                       # Do t1 := EP1(e)
    roriw  a6, s4, 11
    xor    a5, a5, a6
    roriw  a6, s4, 25
    xor    a5, a5, a6
    add    a5, a5, s7                      # Do t1 += h
    and    a6, s5, s4                      # Do t1 += Ch(e,f,g)
    andn   a7, s6, s4
    or     a6, a6, a7
    add    a5, a5, a6
    lw     a6, 0(a2)                       # Do t1 += m[i]
    add    a5, a5, a6
    lw     a6, 0(a3)                       # Do t1 += k[i]
    add    a5, a5, a6
    roriw  a6, s0, 2                       # Do t2 := EP0(a)
    roriw  a7, s0, 13
    xor    a6, a6, a7
    roriw  a7, s0, 22
    xor    a6, a6, a7
    xor    a7, s1, s2                      # Do t2 += Maj(a,b,c)
    and    a7, s0, a7
    and    a1, s1, s2
    xor    a7, a7, a1
    add    a6, a6, a7
    mv     s7, s6                          # Do h=g
    mv     s6, s5                          # Do g=f
    mv     s5, s4                          # Do f=e
    add    s4, s3, a5                      # Do e=d+t1
    mv     s3, s2                          # Do d=c
    mv     s2, s1                          # Do c=b
    mv     s1, s0                          # Do b=a
    add    s0, a5, a6                      # Do a=t1+t2
    addi   a2, a2, 4
    addi   a3, a3, 4
    bne    a3, a4, .sha256_transform_loop_3

    add    t0, t0, s0
    sw     t0, 80(a0)
    add    t1, t1, s1
    sw     t1, 84(a0)
    add    t2, t2, s2
    sw     t2, 88(a0)
    add    t3, t3, s3
    sw     t3, 92(a0)
    add    t4, t4, s4
    sw     t4, 96(a0)
    add    t5, t5, s5
    sw     t5, 100(a0)
    add    t6, t6, s6
    sw     t6, 104(a0)
    add    s8, s8, s7
    sw     s8, 108(a0)

    ld     s0, 328(sp)
    ld     s1, 320(sp)
    ld     s2, 312(sp)
    ld     s3, 304(sp)
    ld     s4, 296(sp)
    ld     s5, 288(sp)
    ld     s6, 280(sp)
    ld     s7, 272(sp)
    ld     s8, 264(sp)
    addi   sp, sp, 336
    ret

.section .rodata
.sha256_init_data:
    .quad 0xbb67ae856a09e667
    .quad 0xa54ff53a3c6ef372
    .quad 0x9b05688c510e527f
    .quad 0x5be0cd191f83d9ab
.text
.globl sha256_init
sha256_init:
    sw     zero, 64(a0)
    sd     zero, 72(a0)
    la     a1, .sha256_init_data
    ld     a2, 0(a1)
    sd     a2, 80(a0)
    ld     a2, 8(a1)
    sd     a2, 88(a0)
    ld     a2, 16(a1)
    sd     a2, 96(a0)
    ld     a2, 24(a1)
    sd     a2, 104(a0)
    ret

.text
.globl sha256_update
sha256_update:
    beqz   a2, .sha256_update_done
    addi   sp, sp, -48
    sd	   ra, 40(sp)
    sd 	   s0, 32(sp)
    sd 	   s1, 24(sp)
    sd     s2, 16(sp)
    sd 	   s3, 8(sp)
    sd	   s4, 0(sp)
    lwu	   s0, 64(a0)                      # Let s0 = ctx->datalen
    mv	   s1, a0                          # Let s1 = ctx
    mv	   s2, a1                          # Let s2 = data
    add	   s3, a1, a2                      # Let s3 = data + len (end pointer)
    li	   s4, 64                          # Let s4 = 64 (block size constant)
    j	   .sha256_update_body
.sha256_update_cond:
    addi   s2, s2, 1
    bgeu   s2, s3, .sha256_update_post
.sha256_update_body:
    lbu	   a0, 0(s2)                       # Load byte from data and store in ctx->data[]
    add	   a1, s1, s0
    sb	   a0, 0(a1)
    addiw  s0, s0, 1                       # Datalen: increment
    bne    s0, s4, .sha256_update_cond
    mv     a0, s1
    mv     a1, s1
    call   sha256_transform
    ld	   a0, 72(s1)
    addi   a0, a0, 512
    sd	   a0, 72(s1)
    mv	   s0, zero                        # Datalen: reset to 0
    j	   .sha256_update_cond
.sha256_update_post:
    sw 	   s0, 64(s1)                      # Datalen: store back
    ld 	   ra, 40(sp)
    ld 	   s0, 32(sp)
    ld	   s1, 24(sp)
    ld	   s2, 16(sp)
    ld 	   s3, 8(sp)
    ld	   s4, 0(sp)
    addi   sp, sp, 48
.sha256_update_done:
    ret

.text
.globl sha256_final
sha256_final:
    addi   sp, sp, -32
    sd     ra, 24(sp)
    sd     s0, 16(sp)
    sd     s1, 8(sp)
    mv     s0, a0                         # Let s0 = ctx
    mv	   s1, a1                         # Let s1 = hash output buffer
    lwu	   a2, 64(a0)                     # Let a2 = ctx->datalen
    add	   a0, a0, a2                     # Let a0 = ctx->data + datalen
    li	   a1, 128
    sb	   a1, 0(a0)
    li	   a1, 55
    bltu   a1, a2, .sha256_final_ge_56
    beq	   a2, a1, .sha256_final_eq_56
    addi   a0, a0, 1
    li	   a1, 55
    subw   a2, a1, a2
    li	   a1, 0
    call   memset
    j	   .sha256_final_eq_56
.sha256_final_ge_56:
    addiw  a0, a2, 1
    li	   a1, 63
    bltu   a1, a0, .sha256_final_eq_64
    add	   a0, a0, s0
    li	   a1, 62
    subw   a1, a1, a2
    addi   a2, a1, 1
    li	   a1, 0
    call   memset
.sha256_final_eq_64:
    mv	   a0, s0
    mv	   a1, s0
    call   sha256_transform
    sd	   zero, 48(s0)
    sd	   zero, 40(s0)
    sd	   zero, 32(s0)
    sd	   zero, 24(s0)
    sd	   zero, 16(s0)
    sd	   zero, 8(s0)
    sd	   zero, 0(s0)
.sha256_final_eq_56:
    lw	   a0, 64(s0)
    ld	   a1, 72(s0)
    slli   a0, a0, 3
    add.uw a0, a0, a1
    sd	   a0, 72(s0)
    rev8   a0, a0                         # Reverse bytes for big-endian
    sd	   a0, 56(s0)                     # Store all 8 bytes at once
    mv	   a0, s0
    mv	   a1, s0
    call   sha256_transform
    ld	   a0, 80(s0)                     # Load state[0] (low) and state[1] (high)
    rori   a0, a0, 32                     # Rotate right 32 bits to swap the two words
    rev8   a0, a0                         # Reverse bytes to big-endian
    sd	   a0, 0(s1)                      # Store state[0] then state[1] in big-endian
    ld	   a0, 88(s0)                     # Load state[2] and state[3]
    rori   a0, a0, 32
    rev8   a0, a0
    sd	   a0, 8(s1)
    ld	   a0, 96(s0)                     # Load state[4] and state[5]
    rori   a0, a0, 32
    rev8   a0, a0
    sd	   a0, 16(s1)
    ld	   a0, 104(s0)                    # Load state[6] and state[7]
    rori   a0, a0, 32
    rev8   a0, a0
    sd	   a0, 24(s1)
    ld	   ra, 24(sp)
    ld	   s0, 16(sp)
    ld	   s1, 8(sp)
    addi   sp, sp, 32
    ret
