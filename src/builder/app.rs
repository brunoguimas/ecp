use crate::builder::Command;

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
}
