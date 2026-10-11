//! hash.rs
//! Fingerprint hash computation module. Handles string inputs only and does not
//! depend on the hardware module.
//! Responsibility: hardware raw string + app_salt -> SHA256 hash; generate hex apply code.
use hex::decode;
use sha2::{Digest, Sha256};

/// Computes the salted hardware fingerprint SHA256.
/// # Arguments
/// * `hw_raw` - the string produced by `HardwareRawInfo.to_raw_string()`
/// * `app_salt` - per-product salt, case-sensitive
/// # Returns
/// SHA256 digest as `[u8; 32]`
pub fn compute_fp_hash_with_salt(hw_raw: &str, app_salt: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let combined = format!("{}{}", hw_raw, app_salt);
    hasher.update(combined.as_bytes());
    let digest = hasher.finalize();
    digest.into()
}

/// Generates the apply code (hex string).
/// # Arguments
/// * `hw_raw` - the string produced by `HardwareRawInfo.to_raw_string()`
/// * `app_salt` - per-product salt
pub fn get_apply_code_with_salt(hw_raw: &str, app_salt: &str) -> String {
    let hash_arr = compute_fp_hash_with_salt(hw_raw, app_salt);
    hex::encode(hash_arr)
}

/// Converts an apply code (hex string) back to `[u8; 32]`.
/// The apply code is always a SHA256 output: 64 hex chars -> 32 bytes.
/// # Errors
/// Returns `Err` if hex decoding fails or the decoded length is not 32 bytes.
pub fn apply_code_to_hash_bytes(apply_code_hex: &str) -> Result<[u8; 32], String> {
    let bytes = decode(apply_code_hex).map_err(|e| format!("hex decode error: {}", e))?;
    let len = bytes.len();
    if bytes.len() != 32 {
        return Err(format!(
            "apply code length invalid, expect 32 bytes, got {}",
            len
        ));
    }
    bytes
        .try_into()
        .map_err(|_| "apply code convert to [u8;32] failed".to_string())
}