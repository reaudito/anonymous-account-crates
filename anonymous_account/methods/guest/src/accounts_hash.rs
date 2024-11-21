use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sp_io::hashing::blake2_256;
use subxt_core::utils::AccountId32;

#[derive(Serialize, Deserialize)]
pub struct AccountData {
    pub account_addresses: Vec<(AccountId32, Vec<u8>)>,
    pub current_hash: [u8; 32],
    pub index: usize,
    pub public_key_of_account: [u8; 32],
    pub password: String,
}

pub fn calculate_hash_for_accounts(accounts: &[(AccountId32, Vec<u8>)]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let mut input_data = Vec::new();

    // Concatenate all account IDs and ByteArray64 contents into a single byte vector
    for account in accounts {
        input_data.extend_from_slice(account.0.as_ref()); // AccountId32 as bytes
        input_data.extend_from_slice(account.1.as_ref());
    }
    hasher.update(&input_data);
    // Compute the hash of the combined data
    let hash = hasher.finalize();
    let hash_bytes: [u8; 32] = hash.into(); // Convert to [u8; 32]
    hash_bytes
}

pub fn calculate_hash_for_accounts2(accounts: &[(AccountId32, Vec<u8>)]) -> [u8; 32] {
    let mut input_data = Vec::new();

    // Concatenate all account IDs and ByteArray64 contents into a single byte vector
    for account in accounts {
        input_data.extend_from_slice(account.0.as_ref()); // AccountId32 as bytes
        input_data.extend_from_slice(account.1.as_ref());
    }

    // Compute the hash of the combined data
    blake2_256(&input_data)
}
pub fn password_hash_fn(index: usize, password: String) -> [u8; 32] {
    // Convert the index to bytes (using 8 bytes to represent usize)
    let mut index_bytes = index.to_le_bytes().to_vec();

    // Convert the password string to bytes and append to index bytes
    index_bytes.extend(password.as_bytes());
    // Hash the combined data using blake2_256
    blake2_256(&index_bytes)
}
