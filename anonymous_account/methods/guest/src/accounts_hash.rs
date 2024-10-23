use serde::{Deserialize, Serialize};
use sp_io::hashing::blake2_256;
use subxt_core::utils::AccountId32;

#[derive(Serialize, Deserialize)]
pub struct AccountData {
    pub account_addresses: Vec<AccountId32>,
    pub current_hash: [u8; 32],
    pub index: usize,
    pub public_key_of_account: [u8; 32],
    pub signature: Vec<u8>, // Use Vec<u8> instead of [u8; 64]
    pub password: String,
}

pub fn update_hash_incrementally(current_hash: [u8; 32], account_id: &AccountId32) -> [u8; 32] {
    let mut input_data = Vec::new();

    // Extend input data with the current hash and the new account ID
    input_data.extend_from_slice(&current_hash);
    input_data.extend_from_slice(account_id.as_ref());

    // Recalculate the hash with the new account ID
    blake2_256(&input_data)
}
