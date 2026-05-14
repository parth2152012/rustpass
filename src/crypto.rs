use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};

use argon2::{Argon2, password_hash::SaltString};

use base64::{Engine, engine::general_purpose};

use rand::{RngCore, rngs::OsRng};

use std::error::Error;
use std::io;

// --------------------------------------------------
// Derive 32-byte encryption key using Argon2
// --------------------------------------------------
pub fn derive_key(password: &str, salt: &str) -> Result<[u8; 32], Box<dyn Error>> {
    // 32-byte AES-256 key buffer
    let mut key = [0u8; 32];

    let argon2 = Argon2::default();

    // Fill key buffer using Argon2
    argon2
        .hash_password_into(password.as_bytes(), salt.as_bytes(), &mut key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    Ok(key)
}

// --------------------------------------------------
// Encrypt plaintext data using AES-256-GCM
// --------------------------------------------------
pub fn encrypt_data(plaintext: &str, key: &[u8; 32]) -> Result<String, Box<dyn Error>> {
    // Create AES cipher
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // AES-GCM uses 12-byte nonce
    let mut nonce_bytes = [0u8; 12];

    // Generate secure random nonce
    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt plaintext
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Store:
    // nonce + ciphertext
    let mut combined = Vec::new();

    combined.extend_from_slice(&nonce_bytes);

    combined.extend_from_slice(&ciphertext);

    // Convert binary -> base64 string
    Ok(general_purpose::STANDARD.encode(combined))
}

// --------------------------------------------------
// Decrypt AES-256-GCM encrypted data
// --------------------------------------------------
pub fn decrypt_data(encrypted_data: &str, key: &[u8; 32]) -> Result<String, Box<dyn Error>> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Decode base64 -> bytes
    let combined = general_purpose::STANDARD.decode(encrypted_data)?;

    // First 12 bytes = nonce
    let (nonce_bytes, ciphertext) = combined.split_at(12);

    let nonce = Nonce::from_slice(nonce_bytes);

    // Decrypt ciphertext
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Convert bytes -> UTF-8 String
    Ok(String::from_utf8(plaintext)?)
}

// --------------------------------------------------
// Generate random salt
// --------------------------------------------------
pub fn generate_salt() -> String {
    SaltString::generate(&mut OsRng).to_string()
}
