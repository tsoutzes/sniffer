use sniffer_cli::{Command, CommandType};

#[test]
fn test_command_type_from_arg() {
    // Test short flags with dash
    assert_eq!(CommandType::from_arg("-h"), Some(CommandType::Help));
    assert_eq!(CommandType::from_arg("-i"), Some(CommandType::Interface));
    assert_eq!(CommandType::from_arg("-w"), Some(CommandType::Watch));

    // Test long flags with double dash
    assert_eq!(CommandType::from_arg("--help"), Some(CommandType::Help));
    assert_eq!(
        CommandType::from_arg("--interface"),
        Some(CommandType::Interface)
    );
    assert_eq!(CommandType::from_arg("--watch"), Some(CommandType::Watch));

    // Test without dashes (although this might not be used in your CLI)
    assert_eq!(CommandType::from_arg("help"), Some(CommandType::Help));
    assert_eq!(
        CommandType::from_arg("interface"),
        Some(CommandType::Interface)
    );
    assert_eq!(CommandType::from_arg("watch"), Some(CommandType::Watch));

    // Test invalid commands
    assert_eq!(CommandType::from_arg("invalid"), None);
    assert_eq!(CommandType::from_arg("-invalid"), None);
    assert_eq!(CommandType::from_arg("--invalid"), None);
    assert_eq!(CommandType::from_arg(""), None);
}

#[test]
fn test_command_parse_args_empty() {
    // Test with empty args
    let empty: Vec<String> = vec![];
    assert!(Command::parse_args(&empty).is_none());
}

#[test]
fn test_command_parse_args_no_flags() {
    // Test with args that don't start with dash
    let args = vec!["program".to_string(), "file.txt".to_string()];
    assert!(Command::parse_args(&args).is_none());
}

#[test]
fn test_flags_function() {
    assert_eq!(CommandType::Help.flags(), ("h", "help"));
    assert_eq!(CommandType::Interface.flags(), ("i", "interface"));
    assert_eq!(CommandType::Watch.flags(), ("w", "watch"));
}

#[test]
fn test_description_function() {
    assert_eq!(CommandType::Help.description(), "Display help information");
    assert_eq!(
        CommandType::Interface.description(),
        "Configure network interface"
    );
    assert_eq!(CommandType::Watch.description(), "Monitor network packets");
}
