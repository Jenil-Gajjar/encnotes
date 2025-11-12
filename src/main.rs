use anyhow::Result;

mod cli;
mod crypto;
mod session;
mod vault;

use clap::Parser;
use cli::*;

use crate::session::save_session;
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login => {
            let password = crypto::prompt_password("Enter Master Password: ")?;
            if std::path::Path::new("vault.enc").exists() {
                match vault::verify_password(&password) {
                    Ok(_) => save_session(&password)?,
                    Err(e) => {
                        println!("Error: {e}");
                        return Ok(());
                    }
                }
            } else {
                session::save_session(&password)?;
                println!("Logged in. No vault found yet (use `init` to create one).");
            }
        }

        Commands::Logout => {
            session::clear_session()?;
            println!("Logged out and cleared session password.");
        }
        _ => {
            let password = match session::load_session()? {
                Some(p) => p,
                None => crypto::prompt_password("Enter Master Password: ")?,
            };

            match cli.command {
                Commands::Init => vault::init_vault(&password)?,
                Commands::List => vault::list_notes(&password)?,
                Commands::Add => vault::add_note(&password)?,
                Commands::Update => vault::edit_note(&password)?,
                Commands::View { query } => vault::view_note(&query, &password)?,
                Commands::Delete { id } => vault::delete_note(&id, &password)?,
                Commands::ChangePwd => {
                    let new_password = crypto::prompt_password("Enter new master password: ")?;
                    let confirm = crypto::prompt_password("Confirm new master password: ")?;
                    if new_password != confirm {
                        println!("Passwords do not match.");
                        return Ok(());
                    }
                    vault::change_password(&password, &new_password)?;

                    if session::load_session()?.is_some() {
                        save_session(&new_password)?;
                    }

                    println!("Password updated successfully.");
                }
                _ => unreachable!(),
            }
        }
    }
    Ok(())
}
