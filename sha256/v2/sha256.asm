	.text
	.attribute	4, 16
	.attribute	5, "rv64i2p1_m2p0_c2p0_zmmul1p0_zba1p0_zbb1p0_zbc1p0_zbs1p0"
	.file	"sha256.c"
	.globl	sha256_transform                # -- Begin function sha256_transform
	.p2align	1
	.type	sha256_transform,@function
sha256_transform:                       # @sha256_transform
# %bb.0:
	addi	sp, sp, -336
	sd	s0, 328(sp)                     # 8-byte Folded Spill
	sd	s1, 320(sp)                     # 8-byte Folded Spill
	sd	s2, 312(sp)                     # 8-byte Folded Spill
	sd	s3, 304(sp)                     # 8-byte Folded Spill
	sd	s4, 296(sp)                     # 8-byte Folded Spill
	sd	s5, 288(sp)                     # 8-byte Folded Spill
	sd	s6, 280(sp)                     # 8-byte Folded Spill
	sd	s7, 272(sp)                     # 8-byte Folded Spill
	sd	s8, 264(sp)                     # 8-byte Folded Spill
	addi	a2, a1, 3
	addi	a3, sp, 8
	addi	a1, sp, 72
.LBB0_1:                                # =>This Inner Loop Header: Depth=1
	lbu	a4, -3(a2)
	lbu	a5, -2(a2)
	slli	a4, a4, 24
	lbu	s1, -1(a2)
	lbu	s0, 0(a2)
	slli	a5, a5, 16
	or	a4, a4, a5
	slli	s1, s1, 8
	or	s0, s0, s1
	or	a4, a4, s0
	sw	a4, 0(a3)
	addi	a3, a3, 4
	addi	a2, a2, 4
	bne	a3, a1, .LBB0_1
# %bb.2:
	lw	a3, 8(sp)
	addi	a6, sp, 264
.LBB0_3:                                # =>This Inner Loop Header: Depth=1
	lw	a4, -8(a1)
	lw	a5, -28(a1)
	roriw	s1, a4, 17
	roriw	s0, a4, 19
	add	a2, a5, a3
	lwu	a3, -60(a1)
	xor	s0, s0, s1
	srliw	a4, a4, 10
	xor	a4, a4, s0
	roriw	s1, a3, 7
	roriw	s0, a3, 18
	xor	s0, s0, s1
	srli	a5, a3, 3
	xor	a5, a5, s0
	add	a2, a2, a4
	add	a2, a2, a5
	sw	a2, 0(a1)
	addi	a1, a1, 4
	bne	a1, a6, .LBB0_3
# %bb.4:
	lw	t5, 80(a0)
	lw	t4, 84(a0)
	lw	t3, 88(a0)
	lw	t2, 92(a0)
	lw	t1, 96(a0)
	lw	t0, 100(a0)
	lw	a7, 104(a0)
	lw	a6, 108(a0)
	lui	a4, %hi(k)
	addi	a4, a4, %lo(k)
	addi	a5, sp, 8
	addi	t6, a4, 256
	mv	s7, t5
	mv	a1, t4
	mv	s5, a6
	mv	a3, a7
	mv	a2, t0
	mv	s8, t1
	mv	s4, t2
	mv	s1, t3
.LBB0_5:                                # =>This Inner Loop Header: Depth=1
	mv	s6, s1
	mv	s0, s8
	mv	s3, a2
	mv	s2, a3
	mv	s1, a1
	mv	a1, s7
	roriw	a2, s8, 6
	roriw	a3, s8, 11
	xor	a2, a2, a3
	roriw	a3, s8, 25
	xor	s7, a2, a3
	and	a3, s3, s8
	andn	a2, s2, s8
	or	s8, a2, a3
	lw	a3, 0(a4)
	lw	a2, 0(a5)
	add	s5, s5, s8
	add	s5, s5, s7
	add	a3, a3, s5
	add	a2, a2, a3
	roriw	s5, a1, 2
	roriw	a3, a1, 13
	xor	s5, s5, a3
	roriw	a3, a1, 22
	xor	s5, s5, a3
	xor	a3, s1, s6
	and	s7, a1, a3
	and	a3, s1, s6
	xor	a3, s7, a3
	add	a3, a3, s5
	add	s8, a2, s4
	add	s7, a3, a2
	addi	a4, a4, 4
	addi	a5, a5, 4
	mv	s5, s2
	mv	a3, s3
	mv	a2, s0
	mv	s4, s6
	bne	a4, t6, .LBB0_5
# %bb.6:
	add	t5, t5, s7
	sw	t5, 80(a0)
	add	a1, a1, t4
	sw	a1, 84(a0)
	add	t3, t3, s1
	sw	t3, 88(a0)
	add	t2, t2, s6
	sw	t2, 92(a0)
	add	t1, t1, s8
	sw	t1, 96(a0)
	add	t0, t0, s0
	sw	t0, 100(a0)
	add	a7, a7, s3
	sw	a7, 104(a0)
	add	a6, a6, s2
	sw	a6, 108(a0)
	ld	s0, 328(sp)                     # 8-byte Folded Reload
	ld	s1, 320(sp)                     # 8-byte Folded Reload
	ld	s2, 312(sp)                     # 8-byte Folded Reload
	ld	s3, 304(sp)                     # 8-byte Folded Reload
	ld	s4, 296(sp)                     # 8-byte Folded Reload
	ld	s5, 288(sp)                     # 8-byte Folded Reload
	ld	s6, 280(sp)                     # 8-byte Folded Reload
	ld	s7, 272(sp)                     # 8-byte Folded Reload
	ld	s8, 264(sp)                     # 8-byte Folded Reload
	addi	sp, sp, 336
	ret
.Lfunc_end0:
	.size	sha256_transform, .Lfunc_end0-sha256_transform
                                        # -- End function
	.section	.srodata.cst8,"aM",@progbits,8
	.p2align	3, 0x0                          # -- Begin function sha256_init
.LCPI1_0:
	.quad	-4942790177982912921            # 0xbb67ae856a09e667
.LCPI1_1:
	.quad	-6534734903820487822            # 0xa54ff53a3c6ef372
.LCPI1_2:
	.quad	-7276294671082564993            # 0x9b05688c510e527f
.LCPI1_3:
	.quad	6620516960021240235             # 0x5be0cd191f83d9ab
	.text
	.globl	sha256_init
	.p2align	1
	.type	sha256_init,@function
sha256_init:                            # @sha256_init
# %bb.0:
	lui	a1, %hi(.LCPI1_0)
	ld	a1, %lo(.LCPI1_0)(a1)
	lui	a2, %hi(.LCPI1_1)
	ld	a2, %lo(.LCPI1_1)(a2)
	sd	a1, 80(a0)
	lui	a1, %hi(.LCPI1_2)
	ld	a1, %lo(.LCPI1_2)(a1)
	sd	a2, 88(a0)
	lui	a2, %hi(.LCPI1_3)
	ld	a2, %lo(.LCPI1_3)(a2)
	sd	a1, 96(a0)
	sw	zero, 64(a0)
	sd	zero, 72(a0)
	sd	a2, 104(a0)
	ret
.Lfunc_end1:
	.size	sha256_init, .Lfunc_end1-sha256_init
                                        # -- End function
	.globl	sha256_update                   # -- Begin function sha256_update
	.p2align	1
	.type	sha256_update,@function
sha256_update:                          # @sha256_update
# %bb.0:
	beqz	a2, .LBB2_6
# %bb.1:
	addi	sp, sp, -48
	sd	ra, 40(sp)                      # 8-byte Folded Spill
	sd	s0, 32(sp)                      # 8-byte Folded Spill
	sd	s1, 24(sp)                      # 8-byte Folded Spill
	sd	s2, 16(sp)                      # 8-byte Folded Spill
	sd	s3, 8(sp)                       # 8-byte Folded Spill
	sd	s4, 0(sp)                       # 8-byte Folded Spill
	mv	s3, a2
	mv	s2, a1
	mv	s1, a0
	lw	a0, 64(a0)
	li	a1, 0
	li	s0, 1
	li	s4, 64
	j	.LBB2_3
.LBB2_2:                                #   in Loop: Header=BB2_3 Depth=1
	zext.w	a1, s0
	addi	s0, s0, 1
	bgeu	a1, s3, .LBB2_5
.LBB2_3:                                # =>This Inner Loop Header: Depth=1
	add	a1, a1, s2
	lbu	a1, 0(a1)
	add.uw	a0, a0, s1
	sb	a1, 0(a0)
	lw	a0, 64(s1)
	addiw	a0, a0, 1
	sw	a0, 64(s1)
	bne	a0, s4, .LBB2_2
# %bb.4:                                #   in Loop: Header=BB2_3 Depth=1
	mv	a0, s1
	mv	a1, s1
	call	sha256_transform
	ld	a1, 72(s1)
	li	a0, 0
	addi	a1, a1, 512
	sd	a1, 72(s1)
	sw	zero, 64(s1)
	j	.LBB2_2
.LBB2_5:
	ld	ra, 40(sp)                      # 8-byte Folded Reload
	ld	s0, 32(sp)                      # 8-byte Folded Reload
	ld	s1, 24(sp)                      # 8-byte Folded Reload
	ld	s2, 16(sp)                      # 8-byte Folded Reload
	ld	s3, 8(sp)                       # 8-byte Folded Reload
	ld	s4, 0(sp)                       # 8-byte Folded Reload
	addi	sp, sp, 48
.LBB2_6:
	ret
.Lfunc_end2:
	.size	sha256_update, .Lfunc_end2-sha256_update
                                        # -- End function
	.globl	sha256_final                    # -- Begin function sha256_final
	.p2align	1
	.type	sha256_final,@function
sha256_final:                           # @sha256_final
# %bb.0:
	addi	sp, sp, -32
	sd	ra, 24(sp)                      # 8-byte Folded Spill
	sd	s0, 16(sp)                      # 8-byte Folded Spill
	sd	s1, 8(sp)                       # 8-byte Folded Spill
	mv	s0, a0
	lwu	a2, 64(a0)
	mv	s1, a1
	add	a0, a0, a2
	li	a3, 128
	li	a1, 55
	sb	a3, 0(a0)
	bltu	a1, a2, .LBB3_3
# %bb.1:
	beq	a2, a1, .LBB3_6
# %bb.2:
	addi	a0, a0, 1
	li	a1, 55
	subw	a2, a1, a2
	li	a1, 0
	call	memset
	j	.LBB3_6
.LBB3_3:
	addiw	a0, a2, 1
	li	a1, 63
	bltu	a1, a0, .LBB3_5
# %bb.4:
	add	a0, a0, s0
	li	a1, 62
	subw	a1, a1, a2
	addi	a2, a1, 1
	li	a1, 0
	call	memset
.LBB3_5:
	mv	a0, s0
	mv	a1, s0
	call	sha256_transform
	sd	zero, 48(s0)
	sd	zero, 40(s0)
	sd	zero, 32(s0)
	sd	zero, 24(s0)
	sd	zero, 16(s0)
	sd	zero, 8(s0)
	sd	zero, 0(s0)
.LBB3_6:
	lw	a0, 64(s0)
	ld	a1, 72(s0)
	slli	a0, a0, 3
	add.uw	a0, a0, a1
	sd	a0, 72(s0)
	sb	a0, 63(s0)
	srli	a1, a0, 8
	sb	a1, 62(s0)
	srli	a1, a0, 16
	sb	a1, 61(s0)
	srli	a1, a0, 24
	sb	a1, 60(s0)
	srli	a1, a0, 32
	sb	a1, 59(s0)
	srli	a1, a0, 40
	sb	a1, 58(s0)
	srli	a1, a0, 48
	sb	a1, 57(s0)
	srli	a0, a0, 56
	sb	a0, 56(s0)
	mv	a0, s0
	mv	a1, s0
	call	sha256_transform
	lbu	a0, 83(s0)
	sb	a0, 0(s1)
	lbu	a0, 87(s0)
	sb	a0, 4(s1)
	lbu	a0, 91(s0)
	sb	a0, 8(s1)
	lbu	a0, 95(s0)
	sb	a0, 12(s1)
	lbu	a0, 99(s0)
	sb	a0, 16(s1)
	lbu	a0, 103(s0)
	sb	a0, 20(s1)
	lbu	a0, 107(s0)
	sb	a0, 24(s1)
	lbu	a0, 111(s0)
	sb	a0, 28(s1)
	lbu	a0, 82(s0)
	sb	a0, 1(s1)
	lbu	a0, 86(s0)
	sb	a0, 5(s1)
	lbu	a0, 90(s0)
	sb	a0, 9(s1)
	lbu	a0, 94(s0)
	sb	a0, 13(s1)
	lbu	a0, 98(s0)
	sb	a0, 17(s1)
	lbu	a0, 102(s0)
	sb	a0, 21(s1)
	lbu	a0, 106(s0)
	sb	a0, 25(s1)
	lbu	a0, 110(s0)
	sb	a0, 29(s1)
	lbu	a0, 81(s0)
	sb	a0, 2(s1)
	lbu	a0, 85(s0)
	sb	a0, 6(s1)
	lbu	a0, 89(s0)
	sb	a0, 10(s1)
	lbu	a0, 93(s0)
	sb	a0, 14(s1)
	lbu	a0, 97(s0)
	sb	a0, 18(s1)
	lbu	a0, 101(s0)
	sb	a0, 22(s1)
	lbu	a0, 105(s0)
	sb	a0, 26(s1)
	lbu	a0, 109(s0)
	sb	a0, 30(s1)
	lbu	a0, 80(s0)
	sb	a0, 3(s1)
	lbu	a0, 84(s0)
	sb	a0, 7(s1)
	lbu	a0, 88(s0)
	sb	a0, 11(s1)
	lbu	a0, 92(s0)
	sb	a0, 15(s1)
	lbu	a0, 96(s0)
	sb	a0, 19(s1)
	lbu	a0, 100(s0)
	sb	a0, 23(s1)
	lbu	a0, 104(s0)
	sb	a0, 27(s1)
	lbu	a0, 108(s0)
	sb	a0, 31(s1)
	ld	ra, 24(sp)                      # 8-byte Folded Reload
	ld	s0, 16(sp)                      # 8-byte Folded Reload
	ld	s1, 8(sp)                       # 8-byte Folded Reload
	addi	sp, sp, 32
	ret
.Lfunc_end3:
	.size	sha256_final, .Lfunc_end3-sha256_final
                                        # -- End function
	.type	k,@object                       # @k
	.section	.rodata,"a",@progbits
	.p2align	2, 0x0
k:
	.word	1116352408                      # 0x428a2f98
	.word	1899447441                      # 0x71374491
	.word	3049323471                      # 0xb5c0fbcf
	.word	3921009573                      # 0xe9b5dba5
	.word	961987163                       # 0x3956c25b
	.word	1508970993                      # 0x59f111f1
	.word	2453635748                      # 0x923f82a4
	.word	2870763221                      # 0xab1c5ed5
	.word	3624381080                      # 0xd807aa98
	.word	310598401                       # 0x12835b01
	.word	607225278                       # 0x243185be
	.word	1426881987                      # 0x550c7dc3
	.word	1925078388                      # 0x72be5d74
	.word	2162078206                      # 0x80deb1fe
	.word	2614888103                      # 0x9bdc06a7
	.word	3248222580                      # 0xc19bf174
	.word	3835390401                      # 0xe49b69c1
	.word	4022224774                      # 0xefbe4786
	.word	264347078                       # 0xfc19dc6
	.word	604807628                       # 0x240ca1cc
	.word	770255983                       # 0x2de92c6f
	.word	1249150122                      # 0x4a7484aa
	.word	1555081692                      # 0x5cb0a9dc
	.word	1996064986                      # 0x76f988da
	.word	2554220882                      # 0x983e5152
	.word	2821834349                      # 0xa831c66d
	.word	2952996808                      # 0xb00327c8
	.word	3210313671                      # 0xbf597fc7
	.word	3336571891                      # 0xc6e00bf3
	.word	3584528711                      # 0xd5a79147
	.word	113926993                       # 0x6ca6351
	.word	338241895                       # 0x14292967
	.word	666307205                       # 0x27b70a85
	.word	773529912                       # 0x2e1b2138
	.word	1294757372                      # 0x4d2c6dfc
	.word	1396182291                      # 0x53380d13
	.word	1695183700                      # 0x650a7354
	.word	1986661051                      # 0x766a0abb
	.word	2177026350                      # 0x81c2c92e
	.word	2456956037                      # 0x92722c85
	.word	2730485921                      # 0xa2bfe8a1
	.word	2820302411                      # 0xa81a664b
	.word	3259730800                      # 0xc24b8b70
	.word	3345764771                      # 0xc76c51a3
	.word	3516065817                      # 0xd192e819
	.word	3600352804                      # 0xd6990624
	.word	4094571909                      # 0xf40e3585
	.word	275423344                       # 0x106aa070
	.word	430227734                       # 0x19a4c116
	.word	506948616                       # 0x1e376c08
	.word	659060556                       # 0x2748774c
	.word	883997877                       # 0x34b0bcb5
	.word	958139571                       # 0x391c0cb3
	.word	1322822218                      # 0x4ed8aa4a
	.word	1537002063                      # 0x5b9cca4f
	.word	1747873779                      # 0x682e6ff3
	.word	1955562222                      # 0x748f82ee
	.word	2024104815                      # 0x78a5636f
	.word	2227730452                      # 0x84c87814
	.word	2361852424                      # 0x8cc70208
	.word	2428436474                      # 0x90befffa
	.word	2756734187                      # 0xa4506ceb
	.word	3204031479                      # 0xbef9a3f7
	.word	3329325298                      # 0xc67178f2
	.size	k, 256

	.ident	"Ubuntu clang version 19.1.7 (++20250804090312+cd708029e0b2-1~exp1~20250804210325.79)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
