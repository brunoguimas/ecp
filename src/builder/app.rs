use crate::builder::Command;
use crate::errors::AppError;
use crate::parser::CommandParsed;

pub struct App {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) commands: Vec<Command>,
}

impl App {
    pub fn new(name: &str) -> App {
        App {
            name: name.to_string(),
            version: None,
            description: None,
            commands: Vec::new(),
        }
    }

    pub fn version(mut self, version: &str) -> App {
        self.version = Some(version.to_string());
        self
    }

    pub fn description(mut self, description: &str) -> App {
        self.description = Some(description.to_string());
        self
    }

    pub fn command(mut self, command: Command) -> App {
        self.commands.push(command);
        self
    }

    pub fn parse_args(&self, args: &[String]) -> Result<CommandParsed, AppError> {
        if args.len() < 2 {
            return Err(AppError::InvalidInput("Too few arguments".to_string()));
        }

        let command = get_command(&self, &args)?;
        let subcommand = get_subcommand(&self, &args)?;
        let flags = get_flags(&self, &args)?;
        let values = get_values(&args, &subcommand, &flags)?;

        Ok(CommandParsed {
            command,
            subcommand,
            flags,
            values,
        })
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_version(&self) -> &Option<String> {
        &self.version
    }

    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }

    pub fn get_commands(&self) -> &Vec<Command> {
        &self.commands
    }
}

fn get_command(app: &App, args: &[String]) -> Result<String, AppError> {
    let command_name = &args[1];

    if !app.commands.iter().any(|cmd| cmd.name == *command_name) {
        return Err(AppError::InvalidCommand(format!(
            "Command not found: {}",
            command_name
        )));
    };

    Ok(command_name.to_string())
}

fn get_subcommand(app: &App, args: &[String]) -> Result<Option<String>, AppError> {
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

fn get_flags(app: &App, args: &[String]) -> Result<Vec<String>, AppError> {
    let command_name = &args[1];
    let subcommand_name = &args[2];
    let command = app.commands.iter().find(|cmd| cmd.name == *command_name);

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
            let arg = arg.trim_start_matches("-");

            if let Some(flag) = scope
                .flags
                .iter()
                .find(|flag| flag.long == *arg || flag.short == arg.chars().next())
            {
                found_flags.push(flag);
            };
        }
    }

    if found_flags.len() == 0 {
        return Err(AppError::InvalidFlag(format!("Flags not found")));
    };

    Ok(found_flags.iter().map(|flag| flag.long.clone()).collect())
}

fn get_values(
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
