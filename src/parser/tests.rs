#[cfg(test)]
mod tests {
    use crate::builder::*;

    #[test]
    fn full_app() {
        let args = vec!["run", "cargo", "build", "--release"];

        let cli = App::new("Rust").command(
            Command::new("cargo")
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
                ),
        );
        // .parse_args(args)

        // assert_matches!(parsed.command.as_str(), "Rust");
        // assert_matches!(parsed.subcommands[0].command.as_str(), "cargo")
    }
}
