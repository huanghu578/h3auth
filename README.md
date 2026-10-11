# h3auth
A lightweight, machine-bound licensing crate for Rust using P-256 (secp256r1 / NIST P-256) ECDSA signatures over a salted hardware fingerprint.

##  Overview
h3auth binds a license to a specific machine by:

- Collecting raw hardware identifiers (system UUID, baseboard serial, BIOS serial).

- Hashing them together with a per-product salt using SHA-256.

- Signing that hash with an ECDSA P-256 private key to produce a license file.

- Verifying the license on another run by recomputing the fingerprint and checking the signature against the corresponding public key.

- The crate is designed to be embedded in applications that need simple, offline, hardware-bound license validation.

## Features
Hardware fingerprinting on Windows via WMI (Win32_ComputerSystemProduct, Win32_BaseBoard, Win32_BIOS).

Salt-based isolation so the same machine yields different fingerprints for different products.

ECDSA P-256 signing/verification with keys and signatures exchanged as hex strings.

File-based license generation and verification helpers.

Virtual-machine detection (all-empty hardware identifiers cause generation/verification to be refused).

Pure Rust crypto via the p256 and sha2 crates.

## Project layout

h3auth/  
├── Cargo.toml  
├── src/  
│   ├── lib.rs        # public facade: get_apply_code / generate_auth_file / verify_license*  
│   ├── error.rs      # AuthError and Result alias  
│   ├── hardware.rs   # HardwareRawInfo: raw identifier collection  
│   ├── hash.rs       # SHA-256 salted fingerprint, apply code helpers  
│   └── sign.rs       # P-256 key generation, sign_hash, verify_hash  
└── README.md  

## Installation
Add the crate to your Cargo.toml:

toml
[dependencies]
h3auth = "0.1"
On Windows, the wmi dependency is pulled in automatically.

## Usage
### Issuing side (vendor)
Generate a key pair once and keep the private key safe:


```rust
use h3auth::sign::generate_key_pair;

let (sk_hex, pk_hex) = generate_key_pair();
// Store sk_hex securely; ship pk_hex with the application.
```


On the customer's machine, obtain an apply code:

```rust
use h3auth::get_apply_code;

let apply_code = get_apply_code("MY_PRODUCT_SALT");
println!("Apply code: {}", apply_code);
```
Back on the issuing side,sign the apply code and ship the resulting file:

```rust
use h3auth::generate_auth_file;

let ok = generate_auth_file("license.sig", &apply_code, &sk_hex);
assert!(ok);
```
### Verifying side (application)
```rust
use h3auth::verify_license_from_file;

let valid = verify_license_from_file("license.sig", "MY_PRODUCT_SALT", &pk_hex);
if valid {
    println!("License OK");
} else {
    println!("License invalid");
}
```
You can also verify against an in-memory signature string with verify_license.

## API summary


|  Function  |  Purpose  |    
|  ---  | ---  | ---  |  
|   get_apply_code(app_salt)  |  Returns the hex apply code for this machine. Empty on VM/unknown hardware.  |  
|   generate_auth_file(path, apply_code, private_key)   |   Signs an apply code and writes the signature to path.  |  
|   verify_license(signature, app_salt, pk_hex)   |   Verifies an in-memory signature string.  |  
|   verify_license_from_file(path, app_salt, pk_hex)   |   Reads a signature file and verifies it.  |  
|   hash::compute_fp_hash_with_salt(hw_raw, salt)   |   Raw salted SHA-256 fingerprint. |  
|   hash::get_apply_code_with_salt(hw_raw, salt)   |   Salted fingerprint as a hex apply code.   |  
|   hash::apply_code_to_hash_bytes(apply_code_hex)   |   Decodes a 64-char hex apply code into [u8; 32].   |  
|   sign::generate_key_pair()   |   Generates a P-256 key pair as (sk_hex, pk_hex).   |  
|   sign::sign_hash(sk_hex, hash_bytes)   |   Signs a 32-byte hash.   |  
|   sign::verify_hash(pk_hex, hash_bytes, sig_hex) | Verifies a signature over a 32-byte hash.   |  


## Security notes
app_salt should be a product-specific constant. It is not a secret and does not need to be hidden, but it must match between issuing and verification.

The ECDSA private key must be kept secret. Only the public key ships with the application.

Hardware identifiers (especially BIOS/baseboard serials) can be spoofed or changed on some systems. Treat this as an anti-casual-copy measure, not a cryptographic guarantee of machine identity.

The all-zero UUID and the OEM placeholder "To be filled by O.E.M." are filtered out as invalid.

Virtual machines that expose no real identifiers are refused, causing get_apply_code to return an empty string and verify_license to return false.

## Platform support
Windows: full support via WMI.

Non-Windows: HardwareRawInfo::collect() compiles, but each identifier resolves to "[EMPTY]", so apply-code generation returns an empty string and license verification returns false. If you need Linux/macOS support, extend hardware.rs with platform-specific implementations of get_system_uuid, get_board_sn, and get_bios_sn.

## License
Licensed under either of

- Apache License, Version 2.0

- MIT license

at your option.

## Contributing
Issues and pull requests are welcome. Please run cargo fmt and cargo clippy before submitting.