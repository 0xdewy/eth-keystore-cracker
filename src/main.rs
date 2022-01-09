use anyhow::{Error, Result};
use dotenv;
use eth_keystore;
use ethers::prelude::*;
use num_cpus;
use rand::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use structopt::StructOpt;
mod utils;

#[derive(Debug, StructOpt)]
#[structopt(name = "KeystoreCracker", about = "Ethereum Keystore")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,
    /// Directory of keystore(s) to decrypt
    #[structopt(parse(from_os_str))]
    keystore_directory: PathBuf,
    /// Where new keystores will be saved
    #[structopt(parse(from_os_str))]
    output_directory: PathBuf,
    /// Text file with password on each newline
    #[structopt(parse(from_os_str))]
    passwords_file: PathBuf,
    /// New password for keystore (default: insecure)
    #[structopt(short, long)]
    password: Option<String>,
}

// TODO: keep track of which were decrypted 
pub fn decrypt_and_create_keystore(
    input_dir: &PathBuf,
    output_dir: &PathBuf,
    passwords_file: &PathBuf,
    new_password: String,
) -> Result<()> {
    let cpus = num_cpus::get();

    let passwords = utils::read_lines(passwords_file);

    let thread_share = passwords.len() / cpus + 1;

    let passwords = Arc::new(Mutex::new(passwords));

    let keystores = Arc::new(utils::files_in_directory(input_dir).unwrap());

    let new_password = Arc::new(new_password);

    let mut handles: Vec<JoinHandle<_>> = Vec::new();

    for i in 0..cpus + 1 {
        let out_dir = output_dir.clone();

        let new_pass = new_password.clone();

        let keystores = keystores.clone();

        let mut pass_clone: Vec<String> = Vec::new();

        while pass_clone.len() < thread_share && passwords.lock().unwrap().len() > 0 {
            pass_clone.push(passwords.lock().unwrap().pop().expect("no pass left"));
        }

        if pass_clone.len() > 0 {
            let handle = std::thread::spawn(move || {
                println!("Starting thread {:?} - Trying {:?} passwords", &i, &pass_clone.len());

                for keystore_src in keystores.iter() {
                    for pass in pass_clone.iter() {
                        let out = out_dir.clone();

                        if let Ok(wallet) =
                            LocalWallet::decrypt_keystore(keystore_src.as_path(), pass)
                        {
                            println!("Decrypted wallet: {:?}", &wallet.address());

                            // Generate new keystore with new_password
                            let mut rng = rand::thread_rng();

                            // Construct a 32-byte random private key.
                            let mut private_key = vec![0u8; 32];

                            rng.fill_bytes(private_key.as_mut_slice());

                            let key = eth_keystore::encrypt_key(
                                out,
                                &mut rng,
                                &wallet.signer().to_bytes(),
                                &*new_pass,
                            )
                            .unwrap();

                            println!("Made new keystore: {:?}: ", &key);
                        };
                    }
                }
            });
            handles.push(handle);
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }
    return Ok(());
}

pub async fn try_shuffle(words: Vec<String>) {
    let provider = utils::get_provider().unwrap();
    let mut rng = &mut rand::thread_rng();
    let mut tries: HashMap<String, bool> = HashMap::new();
    loop {
        // collect the results into a vector:
        // TODO: slow
        let shuffled_seed: String = words
            .choose_multiple(&mut rng, words.len())
            .cloned()
            .collect::<Vec<String>>()
            .join(" ");

        if tries.get(&shuffled_seed).is_some() {
            println!("Already seen: {}", &shuffled_seed);
            continue;
        }
        tries.insert(shuffled_seed.clone(), true);
        // TODO: Check against Eth, Btc, Dot, Ksm
        let mnemonic = PathOrString::String(shuffled_seed.clone());
        let wallet = match ethers::signers::MnemonicBuilder::<ethers::signers::coins_bip39::English>::default()
            .phrase(mnemonic.clone())
            .password("insecure")
            .build()
        {
            Ok(w) => w,
            Err(_e) => {
                continue;
            }
        };
        println!("Unlocked wallet: {:?}", wallet);
        if Provider::get_balance(&provider, wallet.address(), None)
            .await
            .unwrap()
            > U256::zero()
        {
            println!(
                "Found balance for wallet: {:?} \nMnemonic: {:?} \n Seed: {:?}",
                wallet.address(),
                &mnemonic,
                &shuffled_seed
            );
            break;
        }
    }
}
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let opt = Opt::from_args();
    let pass = match opt.password {
        Some(p) => p,
        None => "insecure".to_string(),
    };
    decrypt_and_create_keystore(
        &opt.keystore_directory,
        &opt.output_directory,
        &opt.passwords_file,
        pass,
    )
    .unwrap();
}
