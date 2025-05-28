# sphincsplus

Performance test of <https://github.com/sphincs/sphincsplus> on ckb-vm.

```sh
$ ln -s ../../scripts .
$ make SPHINCSPLUS_PARAMS=sphincs-shake-128f SPHINCSPLUS_THASH=simple
$ ckb-debugger --bin sphincsplus_ref
```

- All possible `SPHINCSPLUS_PARAMS` combinations `sphincs-[haraka/shake/sha2]-[128/192/256][f/s]`.
- All possible `SPHINCSPLUS_THASH` combinations `[simple/robust]`.

Use the following command to quickly test all combinations:

```sh
$ make SPHINCSPLUS_PARAMS=sphincs-haraka-128f SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-haraka-128s SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-haraka-192f SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-haraka-192s SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-haraka-256f SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-haraka-256s SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-shake-128f  SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-shake-128s  SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-shake-192f  SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-shake-192s  SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-shake-256f  SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-shake-256s  SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-sha2-128f   SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-sha2-128s   SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-sha2-192f   SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-sha2-192s   SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-sha2-256f   SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-sha2-256s   SPHINCSPLUS_THASH=simple && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-haraka-128f SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-haraka-128s SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-haraka-192f SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-haraka-192s SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-haraka-256f SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-haraka-256s SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-shake-128f  SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-shake-128s  SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-shake-192f  SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-shake-192s  SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-shake-256f  SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-shake-256s  SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-sha2-128f   SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-sha2-128s   SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-sha2-192f   SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-sha2-192s   SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-sha2-256f   SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
$ make SPHINCSPLUS_PARAMS=sphincs-sha2-256s   SPHINCSPLUS_THASH=robust && ckb-debugger --bin sphincsplus_ref
```
