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
        let value = get_value(&self, &args)?;

        Ok(CommandParsed {
            command,
            subcommand,
            flags,
            value,
        })
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

// TODO
fn get_flags(app: &App, args: &[String]) -> Result<Option<Vec<String>>, AppError> {}
