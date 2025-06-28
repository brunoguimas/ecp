#[derive(Debug, Clone)]
pub struct Flag {
    pub(crate) long: String,
    pub(crate) short: Option<char>,
    pub(crate) description: Option<String>,
}

impl Flag {
    pub fn new(long: &str) -> Flag {
        Flag {
            long: long.to_string(),
            short: None,
            description: None,
        }
    }

    pub fn description(mut self, description: &str) -> Flag {
        self.description = Some(description.to_string());
        self
    }

    pub fn short(mut self, short: char) -> Flag {
        self.short = Some(short);
        self
    }

    pub fn get_long(&self) -> &str {
        &self.long
    }

    pub fn get_short(&self) -> Option<char> {
        self.short
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
