pub struct CommandParsed {
    pub command: String,
    pub subcommand: Option<Vec<String>>,
    pub flags: Option<Vec<String>>,
    pub value: Option<String>,
}
