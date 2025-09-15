// Copyright (c) 2025 HomomorphicEncryption.org
// All rights reserved.
//
// This software is licensed under the terms of the Apache v2 License.
// See the LICENSE.md file for details.

use std::env;
use std::fs;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

use tfhe::core_crypto::prelude::*;

fn read_numbers_from_file(filepath: &Path) -> io::Result<Vec<u64>> {
    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(file);

    reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <data size>", args[0]);
        std::process::exit(1); 
    }

    // Parameters
    let lwe_noise_distribution =
        Gaussian::from_dispersion_parameter(StandardDev(0.000007069849454709433), 0.0);
    let ciphertext_modulus: tfhe::core_crypto::prelude::CiphertextModulus<u64> = CiphertextModulus::new_native();
    let message_modulus = 1u64 << 4;
    let delta = (1_u64 << 63) / message_modulus;

    // Load the secret key
    let serialised_data = fs::read("temp/sk.bin")?;
    let lwe_sk: LweSecretKey<Vec<u64>> = bincode::deserialize(&serialised_data)?;

    // Load the input data (LHS and RHS)
    let lhs_cleartext = read_numbers_from_file(&Path::new(&("datasets/".to_owned() + &args[1] + "/lhs.txt")))?;
    let rhs_cleartext = read_numbers_from_file(&Path::new(&("datasets/".to_owned() + &args[1] + "/rhs.txt")))?;
    
    // Generate the RNG
    let mut binder = new_seeder();
    let seeder = binder.as_mut();
    let mut encryption_generator =
        EncryptionRandomGenerator::<DefaultRandomGenerator>::new(seeder.seed(), seeder);

    // Encode and encrypt the LHS
    let lhs_ciphers = lhs_cleartext.into_iter().map(|m| allocate_and_encrypt_new_lwe_ciphertext(
        &lwe_sk,
        Plaintext(m * delta),
        lwe_noise_distribution,
        ciphertext_modulus,
        &mut encryption_generator,
    ));
 
    // Write the LHS
    for (i, cipher) in lhs_ciphers.enumerate() {
        fs::write("temp/cipher_lhs_".to_owned() + &i.to_string() + ".bin", &bincode::serialize(&cipher)?)?
    }
    
    // Encode and encrypt the RHS
    let rhs_ciphers = rhs_cleartext.into_iter().map(|m| allocate_and_encrypt_new_lwe_ciphertext(
        &lwe_sk,
        Plaintext(m * delta),
        lwe_noise_distribution,
        ciphertext_modulus,
        &mut encryption_generator,
    ));

    // Write the RHS
    for (i, cipher) in rhs_ciphers.enumerate() {
        fs::write("temp/cipher_rhs_".to_owned() + &i.to_string() + ".bin", &bincode::serialize(&cipher)?)?
    }

    Ok(())
}
