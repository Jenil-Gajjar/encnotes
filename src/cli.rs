use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "encnotes", version, about = "Encrypted Notes CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Login,
    Logout,
    Init,
    List,
    Add,
    Update,
    View { query: String },
    Delete { id: String },
    ChangePwd,
}
