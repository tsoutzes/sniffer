use core::Command;
use sniffer_cli::{CliDiscovery, CliExecutor};

fn main() {
    let command = Command::discover(&CliDiscovery);

    match command {
        Some(cmd) => {
            if let Err(e) = cmd.execute(&CliExecutor) {
                eprintln!("Error executing command: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("Unknown or missing command. Use -h or --help for usage information.");
            std::process::exit(1);
        }
    }
}
