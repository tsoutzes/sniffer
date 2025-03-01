use sniffer_cli::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match Command::parse_args(&args) {
        Some(cmd) => {
            if let Err(e) = cmd.execute() {
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
