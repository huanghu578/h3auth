//! sign.rs
//! P-256 (secp256r1 / NistP256) ECDSA signing and verification.
//! Keys and signatures are exchanged entirely as hex strings.
#![allow(unused)]
use hex::{decode, encode};
use p256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};
use rand::thread_rng;
use super::error::{Result,AuthError};

/// Generates a P-256 key pair.
/// # Returns
/// A tuple of `(private_key_hex, public_key_hex)`.
pub fn generate_key_pair() -> (String, String) {
    let signing_key = SigningKey::random(&mut thread_rng());
    let verify_key = VerifyingKey::from(&signing_key);
    let sk_hex = encode(signing_key.to_bytes());
    let pk_hex = encode(verify_key.to_sec1_bytes());
    (sk_hex, pk_hex)
}

/// Signs a 32-byte SHA256 hash using ECDSA.
/// # Arguments
/// * `sk_hex` - private key as a hex string
/// * `hash_bytes` - SHA256 output `[u8; 32]`
/// # Returns
/// The signature as a hex string.
pub fn sign_hash(sk_hex: &str, hash_bytes: &[u8; 32]) -> Result<String> {
    let sk_bytes = decode(sk_hex)?;
    let signing_key =
        SigningKey::from_slice(&sk_bytes).map_err(|_| AuthError::InvalidPrivateKey)?;
    let sig: Signature = signing_key.sign(hash_bytes);
    Ok(encode(sig.to_bytes()))
}

/// Verifies a signature.
/// # Arguments
/// * `pk_hex` - public key as a hex string
/// * `hash_bytes` - the original 32-byte SHA256 hash
/// * `sig_hex` - signature as a hex string
pub fn verify_hash(pk_hex: &str, hash_bytes: &[u8; 32], sig_hex: &str) -> Result<()> {
    let pk_bytes = decode(pk_hex)?;
    let vk = VerifyingKey::from_sec1_bytes(&pk_bytes).map_err(|_| AuthError::InvalidPublicKey)?;
    let sig_bytes = decode(sig_hex)?;
    let sig = Signature::from_slice(&sig_bytes).map_err(|_| AuthError::SignatureParse)?;
    vk.verify(hash_bytes, &sig)
        .map_err(|_| AuthError::VerifyFailed)?;
    Ok(())
}