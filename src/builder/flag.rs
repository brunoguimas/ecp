#[derive(Debug)]
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
}