pub struct CommandParsed {
    pub command: String,
    pub subcommand: Option<String>,
    pub flags: Vec<String>,
    pub values: Vec<String>,
}
