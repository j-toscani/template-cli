mod cli;

use clap::Parser;
use cli::Commands;

pub fn handle_commands(commands: &Option<Commands>) -> () {
    match commands {
        Some(Commands::Add { key, path }) => {
            println!("Chose to add key {key} at path {:?}", path)
        }
        Some(Commands::Remove { key, path }) => {
            println!("Chose to remove key {key} at path {:?}", path)
        }
        Some(Commands::Create { key, path }) => {
            println!("Chose to create key {key} at path {:?}", path)
        }
        Some(Commands::List) => {}
        None => {
            println!("No subcommand chosen!")
        }
    }
}
fn main() {
    let cli = cli::Cli::parse();
    handle_commands(&cli.command);
}
