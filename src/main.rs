use keystore_cracker::decrypt_and_create_keystore;
use dotenv;
use std::path::PathBuf;
use structopt::StructOpt;

mod utils;

#[derive(Debug, StructOpt)]
#[structopt(name = "KeystoreCracker", about = "Ethereum Keystore Cracker")]
pub struct Opt {
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

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let opt = Opt::from_args();
    let pass = opt.password.unwrap_or_else(|| "insecure".to_string());
    let start = std::time::Instant::now();

    decrypt_and_create_keystore(
        &opt.keystore_directory,
        &opt.output_directory,
        &opt.passwords_file,
        pass,
    )
    .expect("Failed to decrypt and create keystore");

    let duration = start.elapsed();
    println!(
        "Time elapsed in decrypt_and_create_keystore() is: {:?}",
        duration
    );
}
