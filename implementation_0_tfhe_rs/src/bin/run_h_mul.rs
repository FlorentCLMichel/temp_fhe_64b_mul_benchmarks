// Copyright (c) 2025 HomomorphicEncryption.org
// All rights reserved.
//
// This software is licensed under the terms of the Apache v2 License.
// See the LICENSE.md file for details.

use std::env;
use std::fs;

use tfhe::{ FheUint64, set_server_key, ServerKey };

use implementation_0_tfhe_rs::half_cipher_cipher_mul_64;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the number of inputs from the first argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <data size>", args[0]);
        std::process::exit(1); 
    }
    let data_size = args[1].parse::<usize>()?;

    // Load the server key
    let serialised_data = fs::read("temp/pk.bin")?;
    let server_key: ServerKey = bincode::deserialize(&serialised_data)?;
    set_server_key(server_key);
    
    // Load the LHS input ciphers
    let ciphers_lhs = (0 .. data_size).map(|i|
        bincode::deserialize::<FheUint64>(&fs::read("temp/cipher_lhs_".to_owned() + &i.to_string() + ".bin")?)
    ).collect::<Result<Vec<FheUint64>, Box<bincode::ErrorKind>>>()?;
    
    // Load the RHS input ciphers
    let ciphers_rhs = (0 .. data_size).map(|i|
        bincode::deserialize::<FheUint64>(&fs::read("temp/cipher_rhs_".to_owned() + &i.to_string() + ".bin")?)
    ).collect::<Result<Vec<FheUint64>, Box<bincode::ErrorKind>>>()?;

    // Run the homomorphic multiplications
    let ciphers_out = ciphers_lhs.iter().zip(ciphers_rhs.iter())
                                 .map(|(lhs, rhs)| half_cipher_cipher_mul_64(lhs, rhs))
                                 .collect::<Vec<FheUint64>>();

    // Write the results
    for (i, cipher) in ciphers_out.iter().enumerate() {
        fs::write("temp/cipher_out_".to_owned() + &i.to_string() + ".bin", &bincode::serialize(&cipher)?)?
    }

    Ok(())
}
