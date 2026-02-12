# k256_ecdsa_sp1

Same as k256_ecdsa, but targeting the SP1 platform.

## How to Build

First, install the SP1 toolchain:

```bash
curl -L https://sp1.succinct.xyz | bash
~/.sp1/bin/sp1up -v 6.0.0-beta.1
~/.sp1/bin/cargo-prove prove --version
```

Then build the project:

```bash
~/.sp1/bin/cargo-prove prove build
```
