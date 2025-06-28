use crate::builder::flag::Flag;

#[derive(Debug, Clone)]
pub struct Command {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) subcommands: Vec<Command>,
    pub(crate) flags: Vec<Flag>,
}

impl Command {
    pub fn new(name: &str) -> Command {
        Command {
            name: name.to_string(),
            description: None,
            subcommands: Vec::new(),
            flags: Vec::new(),
        }
    }

    pub fn description(mut self, description: &str) -> Command {
        self.description = Some(description.to_string());
        self
    }

    pub fn subcommand(mut self, subcommand: Command) -> Command {
        self.subcommands.push(subcommand);
        self
    }

    pub fn flag(mut self, flag: Flag) -> Command {
        self.flags.push(flag);
        self
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn get_subcommands(&self) -> &[Command] {
        &self.subcommands
    }

    pub fn get_flags(&self) -> &[Flag] {
        &self.flags
    }
}
