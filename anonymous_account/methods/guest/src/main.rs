use guest_anonymous_account::accounts_hash::update_hash_incrementally;
use risc0_zkvm::guest::env;
use subxt_core::utils::AccountId32;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let (account_addresses, hash): (Vec<AccountId32>, [u8; 32]) = env::read();

    let mut current_hash: [u8; 32] = [0; 32];
    for account_address in account_addresses {
        current_hash = update_hash_incrementally(current_hash, &account_address);
    }

    assert_eq!(current_hash, hash);
    // assert_ne!(current_hash, hash);

    // write public output to the journal
    env::commit(&current_hash);
}
