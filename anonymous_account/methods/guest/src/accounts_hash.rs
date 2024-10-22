use sp_io::hashing::blake2_256;
use subxt_core::utils::AccountId32;

pub fn update_hash_incrementally(current_hash: [u8; 32], account_id: &AccountId32) -> [u8; 32] {
    let mut input_data = Vec::new();

    // Extend input data with the current hash and the new account ID
    input_data.extend_from_slice(&current_hash);
    input_data.extend_from_slice(account_id.as_ref());

    // Recalculate the hash with the new account ID
    blake2_256(&input_data)
}