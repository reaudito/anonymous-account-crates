use subxt_signer::{bip39::Mnemonic, sr25519::Keypair};

pub fn keypair_func() {
    let mut phrases = Vec::new();
    phrases.push("bottom drive obey lake curtain smoke basket hold race lonely fit walk");
    phrases.push("demand toy recycle symptom this arrow pear ribbon orchard large cabin tower");
    phrases.push("repair resist urban upgrade delay security blush vote bean moment current drill");
    phrases
        .push("disagree romance reform wink essence speak sense fence cause reflect sound alarm");
    phrases.push("figure husband please idea captain bulk despair over letter code art mimic");
    phrases.push("regret family similar face thumb magic head team duty web side strike");
    phrases.push("resemble timber picnic stage must video amount price sport help good stable");

    let mut account_addresses = Vec::new();

    for phrase in phrases {
        let mnemonic = Mnemonic::parse(phrase).unwrap();
        let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();
        let account_address = keypair.public_key().to_account_id();
        println!("{:?}", account_address);
        account_addresses.push(account_address);
    }
}
