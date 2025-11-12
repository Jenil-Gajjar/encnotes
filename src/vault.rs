use std::{fs, io::Write};

use crate::crypto;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Vault {
    pub notes: Vec<Note>,
}

const VAULT_FILE: &str = "vault.enc";

fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

fn read_vault(password: &str) -> Result<Vault> {
    if !std::path::Path::new(VAULT_FILE).exists() {
        return Ok(Vault::default());
    };

    let data = fs::read_to_string(VAULT_FILE)?;
    let enc = serde_json::from_str(&data)?;
    let decrypted = crypto::decrypt_vault(&enc, password)?;
    let vault = serde_json::from_str(&decrypted)?;
    Ok(vault)
}

fn write_vault(vault: &Vault, password: &str) -> Result<()> {
    let json_data = serde_json::to_string_pretty(vault)?;
    let enc = crypto::encrypt_vault(&json_data, password)?;
    let data = serde_json::to_string_pretty(&enc)?;
    fs::write(VAULT_FILE, data)?;
    Ok(())
}

pub fn verify_password(password: &str) -> Result<()> {
    let data = fs::read_to_string(VAULT_FILE)?;
    let enc = serde_json::from_str(&data)?;
    crypto::decrypt_vault(&enc, password)?;
    Ok(())
}

pub fn init_vault(password: &str) -> Result<()> {
    if std::path::Path::new(VAULT_FILE).exists() {
        return Err(anyhow!("Vault already exists"));
    }
    let vault = Vault::default();
    write_vault(&vault, password)?;
    println!("Vault initialized successfully.");
    Ok(())
}

pub fn list_notes(password: &str) -> Result<()> {
    let vault = read_vault(password)?;

    if vault.notes.is_empty() {
        println!("No notes found.");
        return Ok(());
    }

    for (i, note) in vault.notes.iter().enumerate() {
        println!("{}. {} [{}]", i + 1, note.title, note.id)
    }
    Ok(())
}

pub fn add_note(password: &str) -> Result<()> {
    let mut vault = read_vault(password)?;

    print!("Title: ");
    std::io::stdout().flush().ok();

    let mut title = String::new();
    std::io::stdin().read_line(&mut title)?;
    let title = title.trim();

    print!("Description: ");
    std::io::stdout().flush().ok();

    let mut description = String::new();
    std::io::stdin().read_line(&mut description)?;
    let description = description.trim();

    let note = Note {
        id: generate_id(),
        title: title.to_string(),
        description: description.to_string(),
        created_at: Utc::now(),
        modified_at: Utc::now(),
    };
    vault.notes.push(note);
    write_vault(&vault, password)?;
    println!("Note saved successfully.");
    Ok(())
}

pub fn view_note(query: &str, password: &str) -> Result<()> {
    let vault = read_vault(password)?;

    let note = vault
        .notes
        .iter()
        .find(|n| n.id.trim() == query.trim() || n.title.trim().eq_ignore_ascii_case(query.trim()));
    if let Some(note) = note {
        println!("\nTitle:{} [{}]", note.title, note.id);
        println!("{}", note.description);
    } else {
        println!("Note not found.")
    }
    Ok(())
}

pub fn delete_note(id: &str, password: &str) -> Result<()> {
    let mut vault = read_vault(password)?;
    let index = vault
        .notes
        .iter()
        .position(|n| n.id.trim() == id.trim())
        .ok_or(anyhow!("Note not found."))?;
    vault.notes.remove(index);
    write_vault(&vault, password)?;
    println!("Note deleted successfully.");
    Ok(())
}

pub fn edit_note(password: &str) -> Result<()> {
    let mut vault = read_vault(password)?;

    print!("Id: ");
    std::io::stdout().flush().ok();

    let mut id = String::new();
    std::io::stdin().read_line(&mut id)?;
    let id = id.trim();

    let note = vault
        .notes
        .iter_mut()
        .find(|n| n.id.trim() == id.trim())
        .ok_or(anyhow!("Note not found."))?;

    print!("Title: ");
    std::io::stdout().flush().ok();

    let mut title = String::new();
    std::io::stdin().read_line(&mut title)?;
    note.title = title.trim().to_string();

    print!("Description: ");
    std::io::stdout().flush().ok();

    let mut description = String::new();
    std::io::stdin().read_line(&mut description)?;
    note.description = description.trim().to_string();

    write_vault(&vault, password)?;
    println!("Note edited successfully.");
    Ok(())
}

pub fn change_password(old_password: &str, new_password: &str) -> Result<()> {
    let vault = read_vault(old_password)?;
    let json_data = serde_json::to_string(&vault)?;
    let new_enc = crypto::encrypt_vault(&json_data, new_password)?;
    let json = serde_json::to_string_pretty(&new_enc)?;
    fs::write(VAULT_FILE, json)?;

    println!("Master password changed successfully.");
    Ok(())
}
