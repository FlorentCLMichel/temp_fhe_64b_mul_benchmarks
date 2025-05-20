# Homomorphic Multiplication Benchmarks

## Scope

This repository contains a reference implementation of encrypted 64-bits multiplication for the Homomorphic Encryption benchmarking group. 

See the [integer multiplication benchmark document](https://docs.google.com/document/d/1HPHmBfDscTtQAiRGlYV3upykSgK_GOkTg-_Sy41SSAQ/edit?usp=sharing) for more information.

## Structure of the repository

### Plaintext implementation

The directory `plaintext_impl` contains a plaintext implementation of the half and full 64-bits multipliers in C. It does not run any operations on encrypted data, and is only meant to be used to check the correctness of other implementations.

It defines two functions (in `plaintext_impl/src/include/mul_plaintext.h`), for the half and full 64-bits multipliers:

```c
uint64_t half_64b_mul(const uint64_t lhs, const uint64_t rhs);
uint128_t full_64b_mul(const uint64_t lhs, const uint64_t rhs);
```



### Reference homomorphic implementation

The reference implementation can be found in `implementation_0_tfhe_rs`. It uses the [tfhe-rs](https://github.com/zama-ai/tfhe-rs) library.

To build the executables, install the [Rust compiler](https://www.rust-lang.org/) and run:

```
cd implementation_0_tfhe_rs && RUSTFLAGS="-Ctarget-cpu=native" cargo build --release
```

The benchmarks can then be run using

```
cd implementation_0_tfhe_rs && ./target/release/half_cipher_plaintext_64
```

and

```
cd implementation_0_tfhe_rs && ./target/release/half_cipher_plaintext_64
```



## Reference implementation benchmark results

The following results were obtained on an Intel(R) Core(TM) i7-9700K CPU @ 3.60GHz. 

|         Benchmark          | Number of multiplications | Runtime (s) |
| :------------------------: | :-----------------------: | :---------: |
| `half_cipher_plaintext_64` |            100            |     140     |
|  `half_cipher_cipher_64`   |            100            |     570     |
| `full_cipher_plaintext_64` |            100            |     670     |
|  `full_cipher_cipher_64`   |            100            |    1,450    |
