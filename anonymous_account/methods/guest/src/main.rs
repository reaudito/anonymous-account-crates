use guest_anonymous_account::accounts_hash::{update_hash_incrementally, AccountData};
use risc0_zkvm::guest::env;
use subxt_core::utils::AccountId32;
use subxt_signer::sr25519::{PublicKey, Signature};
use subxt_signer::{bip39::Mnemonic, sr25519::Keypair};

fn main() {
    // TODO: Implement your guest code here
    let account_data: AccountData = env::read();
    // // read the input
    // let (account_addresses, hash, (index, public_key_of_account_array, signature_array, password)): (
    //     Vec<AccountId32>,
    //     [u8; 32],
    //     (usize, [u8; 32], [u8; 64], String),
    // ) = env::read();

    let account_addresses = account_data.account_addresses.clone();

    let mut current_hash: [u8; 32] = [0; 32];
    for account_address in account_addresses {
        current_hash = update_hash_incrementally(current_hash, &account_address);
    }

    assert_eq!(current_hash, account_data.current_hash);
    // assert_ne!(current_hash, hash);
    //

    // let public_key_of_account = PublicKey(public_key_of_account_array);
    // let signature_of_account = Signature(signature_array);

    // let mnemonic = Mnemonic::parse(&phrase_of_index).unwrap();
    // let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();
    // let account_address_of_your_account = keypair.public_key().to_account_id();

    // assert_eq!(
    //     account_addresses_clone[index],
    //     account_address_of_your_account
    // );

    // write public output to the journal
    env::commit(&current_hash);
}
