use anyhow::Result;
use ethers::prelude::*;
use rand::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::JoinHandle;
use thiserror;

// The error type for this module
#[derive(Debug, thiserror::Error)]
pub enum KeystoreCrackerError {
    #[error("No passwords provided")]
    NoPasswords,
    #[error("No keystore files found")]
    NoKeystoreFiles,
    #[error("Thread failed to join")]
    ThreadFailedToJoin,
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

// The utility functions for this module
mod utils;

// The function that handles individual threads
pub fn create_thread_handler(
    thread_index: usize,
    out_dir: PathBuf,
    new_pass: Arc<String>,
    keystores: Arc<Vec<PathBuf>>,
    pass_clone: Vec<String>,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        println!(
            "Starting thread {:?} - Trying {:?} passwords",
            &thread_index,
            &pass_clone.len()
        );

        let mut rng = rand::thread_rng();

        for keystore_src in keystores.iter() {
            for pass in pass_clone.iter() {
                if let Ok(wallet) = LocalWallet::decrypt_keystore(keystore_src.as_path(), pass) {
                    println!("Decrypted wallet: {:?}", &wallet.address());

                    let mut private_key = vec![0u8; 32];
                    rng.fill_bytes(private_key.as_mut_slice());

                    let key = eth_keystore::encrypt_key(
                        out_dir.clone(),
                        &mut rng,
                        &wallet.signer().to_bytes(),
                        &*new_pass,
                    )
                    .expect("Failed to encrypt key");

                    println!("Made new keystore: {:?}", &key);
                    break;
                }
            }
        }
    })
}

// The main function that performs the keystore decryption and creation
pub fn decrypt_and_create_keystore(
    input_dir: &PathBuf,
    output_dir: &PathBuf,
    passwords_file: &PathBuf,
    new_password: String,
) -> Result<(), KeystoreCrackerError> {
    // Get the number of available CPU cores
    let cpu_count = num_cpus::get();

    // Read the password lines from the file
    let passwords = utils::read_lines(passwords_file);

    if passwords.is_empty() {
        return Err(KeystoreCrackerError::NoPasswords);
    }

    // Find all the keystore files
    let keystores = Arc::new(utils::find_keystore_files(input_dir)?);

    if keystores.is_empty() {
        return Err(KeystoreCrackerError::NoKeystoreFiles);
    }

    println!("Found {:?} keystore files", keystores.len());

    let new_password = Arc::new(new_password);
    let passwords_chunk_size = std::cmp::max(1, passwords.len() / cpu_count);

    // Create a new thread for each chunk of passwords
    let handles: Vec<JoinHandle<_>> = passwords
        .chunks(passwords_chunk_size)
        .enumerate()
        .filter_map(|(thread_index, passwords_chunk)| {
            let password_slice = passwords_chunk.to_vec();

            if !password_slice.is_empty() {
                Some(create_thread_handler(
                    thread_index,
                    output_dir.clone(),
                    Arc::clone(&new_password),
                    Arc::clone(&keystores),
                    password_slice,
                ))
            } else {
                None
            }
        })
        .collect();

    // Join all threads
    for handle in handles {
        handle
            .join()
            .map_err(|_| KeystoreCrackerError::ThreadFailedToJoin)?;
    }

    Ok(())
}
