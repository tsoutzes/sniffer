#[derive(Debug, Clone)]
pub struct Command {
    command_type: CommandType,
    args: Vec<String>,
}

impl Command {
    pub fn new(command_type: CommandType) -> Command {
        Command {
            command_type,
            args: Vec::new(),
        }
    }

    fn with_args(command_type: CommandType, args: Vec<String>) -> Self {
        Command { command_type, args }
    }

    pub fn parse_args(args: &[String]) -> Option<Self> {
        let cmd_args: Vec<String> = args
            .iter()
            .skip_while(|arg| !arg.starts_with("-"))
            .cloned()
            .collect();

        if cmd_args.is_empty() {
            return None;
        }

        let cmd_str = &cmd_args[0];
        if let Some(cmd_type) = CommandType::from_arg(cmd_str) {
            return Some(Command::with_args(cmd_type, cmd_args));
        }
        None
    }

    pub fn execute(&self) -> Result<(), String> {
        match self.command_type {
            CommandType::Help => {
                println!("Available commands:");
                for cmd in [
                    CommandType::Help,
                    CommandType::Interface,
                    CommandType::Watch,
                ]
                .iter()
                {
                    let (short, long) = cmd.flags();
                    println!("  -{}, --{}: {}", short, long, cmd.description());
                }
                Ok(())
            }
            CommandType::Interface => {
                // Logic for interface command
                println!("Configuring interface with args: {:?}", self.args);
                Ok(())
            }
            CommandType::Watch => {
                // Logic for watch command
                println!("Watching network packets with args: {:?}", self.args);
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    Help,
    Interface,
    Watch,
}

impl CommandType {
    pub fn flags(&self) -> (&str, &str) {
        match self {
            CommandType::Help => ("h", "help"),
            CommandType::Interface => ("i", "interface"),
            CommandType::Watch => ("w", "watch"),
        }
    }

    pub fn from_arg(arg: &str) -> Option<Self> {
        match arg {
            "h" | "help" | "-h" | "--help" => Some(CommandType::Help),
            "i" | "interface" | "-i" | "--interface" => Some(CommandType::Interface),
            "w" | "watch" | "-w" | "--watch" => Some(CommandType::Watch),
            _ => None,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            CommandType::Help => "Display help information",
            CommandType::Interface => "Configure network interface",
            CommandType::Watch => "Monitor network packets",
        }
    }
}
