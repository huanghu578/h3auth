//! h3auth
//! High-level licensing/authentication facade tying together hardware
//! fingerprinting, hashing, and ECDSA signing/verification.

use crate::sign::verify_hash;
use std::path::Path;
pub mod hardware;
pub mod hash;
pub mod sign;
pub mod error;

/// Returns the apply code (hex fingerprint) for this machine.
///
/// Collects raw hardware identifiers, salts and hashes them via [`hash::get_apply_code_with_salt`].
/// Returns an empty string when all hardware identifiers are `"[EMPTY]"`,
/// which indicates a virtual machine or an environment where identifiers
/// cannot be read; in that case no apply code is produced.
pub fn get_apply_code(app_salt: &str) -> String {
    let hw_info = hardware::HardwareRawInfo::collect();
    let raw = hw_info.to_raw_string();
    if raw == "[EMPTY]\0[EMPTY]\0[EMPTY]" {
        return String::new(); // virtual machine hardware; return empty apply code
    }
    hash::get_apply_code_with_salt(&raw, app_salt)
}

/// Generates a license file by signing the given apply code.
///
/// Decodes `apply_code` into a 32-byte hash, signs it with `private_key`,
/// and writes the resulting signature to `path`.
/// Returns `true` on success, or `false` if decoding, signing, or writing fails.
pub fn generate_auth_file<P: AsRef<Path>>(path: P,apply_code: &str,private_key:&str) -> bool {
    if let Ok(hash) = hash::apply_code_to_hash_bytes(apply_code) {
        if let Ok(signature) = sign::sign_hash(private_key, &hash) {
            let path_obj = path.as_ref();
            if let Ok(()) = std::fs::write(path_obj, signature) {
                return true;
            }
        }
    }
    false
}

/// Verifies a license from a signature string.
///
/// Recomputes the salted hardware fingerprint for this machine and checks
/// it against `signature` using the public key `pk_hex`.
/// Returns `false` on virtual-machine hardware (all identifiers `"[EMPTY]"`).
pub fn verify_license(signature: &str,app_salt: &str,pk_hex:&str) -> bool {
    let hw_info = hardware::HardwareRawInfo::collect();
    let raw = hw_info.to_raw_string();
    if raw == "[EMPTY]\0[EMPTY]\0[EMPTY]" {
        return false; // virtual machine hardware
    }
    let hash = hash::compute_fp_hash_with_salt(&raw, app_salt);
    verify_hash(pk_hex, &hash, signature).is_ok()
}

/// Verifies a license by reading the signature from a file.
///
/// Returns `false` if the file does not exist, cannot be read, or the
/// contained signature fails verification. See [`verify_license`].
pub fn verify_license_from_file<P: AsRef<Path>>(path: P,app_salt: &str,pk_hex:&str) -> bool {
    let path_obj = path.as_ref();
    let path = Path::new(path_obj);
    if !path.exists() {
        return false;
    }
    if let Ok(signature) = std::fs::read_to_string(path_obj) {
        if verify_license(&signature,app_salt,pk_hex) {
            return true;
        }
    }
    false
}