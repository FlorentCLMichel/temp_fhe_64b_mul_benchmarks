use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint64};
use tfhe::prelude::*;

fn main() {
    let config = ConfigBuilder::default().build();

    // Client-side
    let (client_key, server_key) = generate_keys(config);

    let clear_a = 1u64 << 50;
    let clear_b = (1u64 << 50) + 1;

    let a = FheUint64::encrypt(clear_a, &client_key);
    let b = FheUint64::encrypt(clear_b, &client_key);

    //Server-side
    set_server_key(server_key);
    let result = a * b;

    //Client-side
    let decrypted_result: u64 = result.decrypt(&client_key);

    let clear_result = clear_a.wrapping_mul(clear_b);

    assert_eq!(decrypted_result, clear_result);

    println!("{clear_a} × {clear_b} = {decrypted_result} mod 2**64");
}
