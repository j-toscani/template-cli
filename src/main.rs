mod cli;
mod handlers;

use clap::Parser;
use cli::Commands;

pub fn handle_commands(commands: &Option<Commands>) -> Result<(), std::io::Error> {
    match commands {
        Some(Commands::Add { key, path }) => {
            handlers::add_template(key, path)?
        }
        Some(Commands::Update { key, path }) => {
            handlers::update_template(key, path)?
        }
        Some(Commands::Remove { key }) => {
            handlers::remove_template(key)?
        }
        Some(Commands::List) => {
            handlers::list_templates()?
        }
        None => {
            println!("No subcommand chosen!")
        }
    }

    Ok(())
}
fn main() -> Result<(), std::io::Error> {
    let cli = cli::Cli::parse();
    handle_commands(&cli.command)
}
