
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