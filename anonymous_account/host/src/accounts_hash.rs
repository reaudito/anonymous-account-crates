use sp_io::hashing::blake2_256;
use subxt::config::substrate::AccountId32;
use subxt_signer::{bip39::Mnemonic, sr25519::Keypair};

fn update_hash_incrementally(current_hash: [u8; 32], account_id: &AccountId32) -> [u8; 32] {
    let mut input_data = Vec::new();

    // Extend input data with the current hash and the new account ID
    input_data.extend_from_slice(&current_hash);
    input_data.extend_from_slice(account_id.as_ref());

    // Recalculate the hash with the new account ID
    blake2_256(&input_data)
}

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
    let mut current_hash: [u8; 32] = [0; 32];
    for phrase in phrases {
        let mnemonic = Mnemonic::parse(phrase).unwrap();
        let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();
        let account_address = keypair.public_key().to_account_id();
        println!("{:?}", account_address);
        account_addresses.push(account_address.clone());
        current_hash = update_hash_incrementally(current_hash, &account_address);
    }

    println!("current_hash:{:?}", current_hash);
}
