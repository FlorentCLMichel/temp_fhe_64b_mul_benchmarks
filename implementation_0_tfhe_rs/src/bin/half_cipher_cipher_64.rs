use implementation_0_tfhe_rs::half_cipher_cipher_mul_64;
use rand::random;
use std::time::Instant;

use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint64};
use tfhe::prelude::*;

const NUM_RUNS: usize = 100;

fn main() {
    println!("Half 64b Cipher/Cipher multiplication test...");
    
    let config = ConfigBuilder::default().build();

    // Client-side
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    let start = Instant::now();
    for _ in 0 .. NUM_RUNS {

        let clear_a: u64 = random();
        let clear_b: u64 = random();

        let a = FheUint64::encrypt(clear_a, &client_key);
        let b = FheUint64::encrypt(clear_b, &client_key);

        let result = half_cipher_cipher_mul_64(&a, &b);

        let decrypted_result: u64 = result.decrypt(&client_key);

        let clear_result = clear_a.wrapping_mul(clear_b);

        assert_eq!(decrypted_result, clear_result);
    }
    let duration = start.elapsed();

    println!("Test passed with {NUM_RUNS} muliplications");
    println!("Runtime: {:?}", duration);
}
