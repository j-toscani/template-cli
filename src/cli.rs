use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { key: String, path: PathBuf },
    Remove { key: String, path: PathBuf },
    Create { key: String, path: PathBuf },
    List
}

