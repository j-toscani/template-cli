mod cli;
mod handlers;

use clap::Parser;
use cli::Commands;

/* @todo Add proper error handling */

pub fn handle_commands(commands: &Option<Commands>) -> Result<(), Box<dyn std::error::Error>> {
    match commands {
        Some(Commands::Add { key, path }) => {
            handlers::add_template(key, path)?
        }
        Some(Commands::Remove { key, path }) => {
            println!("Chose to remove key {key} at path {:?}", path)
        }
        Some(Commands::Create { key, path }) => {
            println!("Chose to create key {key} at path {:?}", path)
        }
        Some(Commands::List) => {
            println!("Listing all available templates...")
        }
        None => {
            println!("No subcommand chosen!")
        }
    }

    Ok(())
}
fn main() {
    let cli = cli::Cli::parse();
    handle_commands(&cli.command).expect("Could not perform task.");
}
