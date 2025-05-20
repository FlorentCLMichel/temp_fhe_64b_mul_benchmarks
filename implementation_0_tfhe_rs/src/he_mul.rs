use tfhe::prelude::*;
use tfhe::FheUint64;

pub fn half_cipher_cipher_mul_64(a: &FheUint64, b: &FheUint64) -> FheUint64 
{
    a * b
}

pub fn half_cipher_plaintext_mul_64(a: &FheUint64, b: u64) -> FheUint64 
{
    a * b
}

pub fn full_cipher_cipher_mul_64(a: &FheUint64, b: &FheUint64) -> (FheUint64, FheUint64)
{
    // TODO
    unimplemented!()
}

pub fn full_cipher_plaintext_mul_64(a: &FheUint64, b: u64) -> (FheUint64, FheUint64)
{
    // TODO
    unimplemented!()
}
