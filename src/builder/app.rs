use std::env;

use crate::builder::Command;
use crate::errors::AppError;
use crate::parser::CommandParsed;
use crate::parser::utils::*;

pub struct App {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) commands: Vec<Command>,
}

// Parsing
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

    pub fn run(&self) -> CommandParsed {
        let args: Vec<String> = env::args().collect();

        match self.parse_args(&args) {
            Ok(parsed) => parsed,
            Err(e) => e.exit(),
        }
    }
}

// Getters
impl App {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn get_commands(&self) -> &[Command] {
        &self.commands
    }
}
