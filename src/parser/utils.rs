use crate::builder::*;
use crate::errors::AppError;

pub fn get_command(app: &App, args: &[String]) -> Result<String, AppError> {
    let command_name = &args[1];

    if !app.commands.iter().any(|cmd| cmd.name == *command_name) {
        return Err(AppError::InvalidCommand(format!(
            "Command not found: {}",
            command_name
        )));
    };

    Ok(command_name.to_string())
}

pub fn get_subcommand(app: &App, args: &[String]) -> Result<Option<String>, AppError> {
    let subcommand_name = if let Some(subcmd) = args.get(2) {
        subcmd
    } else {
        return Ok(None);
    };

    for cmd in app.commands.iter() {
        if !cmd.subcommands.iter().any(|subcmd| subcmd.name == args[2]) {
            return Err(AppError::InvalidCommand(format!(
                "Subcommand not found: {}",
                subcommand_name
            )));
        }
    }

    Ok(Some(subcommand_name.to_string()))
}

pub fn get_flags(app: &App, args: &[String]) -> Result<Vec<String>, AppError> {
    let command_name = &args[1];
    let subcommand_name = &args[2];
    let command = app.commands.iter().find(|cmd| cmd.name == *command_name);

    // Searches for a subcommand that matches args[2], if don't find any the scope will be the main command
    let scope = match command
        .unwrap()
        .subcommands
        .iter()
        .find(|subcmd| subcmd.name == *subcommand_name)
    {
        Some(subcmd) => subcmd,
        None => command.unwrap(),
    };

    let mut found_flags = Vec::new();

    for arg in args {
        if arg.starts_with('-') {
            let arg = arg.trim_start_matches('-');

            if let Some(flag) = scope
                .flags
                .iter()
                .find(|flag| flag.long == arg || flag.short == arg.chars().next())
            {
                found_flags.push(flag);
            };
        }
    }

    if found_flags.is_empty() {
        return Err(AppError::InvalidFlag("Flags not found".to_string()));
    };

    Ok(found_flags.iter().map(|flag| flag.long.clone()).collect())
}

pub fn get_values(
    args: &[String],
    subcommand: &Option<String>,
    flags: &[String],
) -> Result<Vec<String>, AppError> {
    let skip_count = if let Some(_) = subcommand {
        3 + flags.len()
    } else {
        2 + flags.len()
    };

    Ok(args.iter().skip(skip_count).map(|s| s.to_owned()).collect())
}
