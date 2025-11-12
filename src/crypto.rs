#![allow(deprecated)]
use anyhow::{Result, anyhow};
use argon2::Argon2;

use base64::{Engine, engine::general_purpose};
use chacha20poly1305::{
    Key, XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit},
};
use rand::Rng;
use rpassword;
use serde::{Deserialize, Serialize};

const KEY_LEN: usize = 32;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedContent {
    pub cipher_text_b64: String,
    pub salt_b64: String,
    pub nonce_b64: String,
}

pub fn prompt_password(prompt: &str) -> Result<String> {
    let password = rpassword::prompt_password(prompt)?;
    Ok(password)
}

fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; KEY_LEN]> {
    let argon2 = Argon2::default();
    let mut key = [0u8; KEY_LEN];
    argon2.hash_password_into(password.as_bytes(), salt, &mut key)?;
    Ok(key)
}

fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    rand::rng().fill(&mut salt);
    salt
}

fn generate_nonce() -> [u8; NONCE_LEN] {
    let mut nonce = [0u8; NONCE_LEN];
    rand::rng().fill(&mut nonce);
    nonce
}

pub fn encrypt_vault(plaintext: &str, password: &str) -> Result<EncryptedContent> {
    let salt = generate_salt();
    let nonce = generate_nonce();
    let key_bytes = derive_key(password, &salt)?;

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key_bytes));
    let cipher_text = cipher
        .encrypt(XNonce::from_slice(&nonce), plaintext.as_bytes())
        .map_err(|e| anyhow!("{e}"))?;

    Ok(EncryptedContent {
        cipher_text_b64: general_purpose::STANDARD.encode(cipher_text),
        salt_b64: general_purpose::STANDARD.encode(salt),
        nonce_b64: general_purpose::STANDARD.encode(nonce),
    })
}

pub fn decrypt_vault(content: &EncryptedContent, password: &str) -> Result<String> {
    let salt = general_purpose::STANDARD.decode(&content.salt_b64)?;
    let nonce = general_purpose::STANDARD.decode(&content.nonce_b64)?;
    let cipher_text = general_purpose::STANDARD.decode(&content.cipher_text_b64)?;
    let key_bytes = derive_key(password, &salt)?;
    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key_bytes));
    let plain_text = cipher
        .decrypt(XNonce::from_slice(&nonce), cipher_text.as_ref())
        .map_err(|_| anyhow!("Wrong Password."))?;

    Ok(String::from_utf8(plain_text)?)
}
