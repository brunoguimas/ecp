pub struct Parsed {
    pub command: String,
    pub subcommands: Vec<Parsed>,
    pub flags: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::builder::*;

    #[test]
    fn full_parsed() {
        let cargo = Command::new("cargo")
            .description("Rust's package manager")
            .subcommand(
                Command::new("build")
                    .description("Compile the current package")
                    .flag(
                        Flag::new("release")
                            .description("Build artifacts in release mode, with optimizations")
                            .short('r'),
                    ),
            )
            .subcommand(
                Command::new("run")
                    .description("Run a binary or example of the local package")
                    .flag(
                        Flag::new("release")
                            .description("Build artifacts in release mode, with optimizations")
                            .short('r'),
                    ),
            );

        let parsed = parse(cargo);
    }
}
