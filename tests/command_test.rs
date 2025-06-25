#[cfg(test)]
mod tests {
    use crate::builder::*;

    #[test]
    fn full_command_build() {
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
            );

        // Command
        assert_matches!(cargo.name.as_str(), "cargo");
        assert_matches!(cargo.description, Some(_));

        // Subcommand
        assert_matches!(cargo.subcommands[0].name.as_str(), "build");
        assert_matches!(cargo.subcommands[0].description, Some(_));

        // Flag
        assert_matches!(cargo.subcommands[0].flags[0].long.as_str(), "release");
        assert_matches!(cargo.subcommands[0].flags[0].description, Some(_));
        assert_matches!(cargo.subcommands[0].flags[0].short, Some(_));
    }

    #[test]
    fn mutiple_subcommands_build() {
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

        // Command
        assert_matches!(cargo.name.as_str(), "cargo");
        assert_matches!(cargo.description, Some(_));

        // Subcommands
        assert_matches!(cargo.subcommands[0].name.as_str(), "build");
        assert_matches!(cargo.subcommands[0].description, Some(_));
        assert_matches!(cargo.subcommands[1].name.as_str(), "run");
        assert_matches!(cargo.subcommands[1].description, Some(_));

        // Flags
        assert_matches!(cargo.subcommands[0].flags[0].long.as_str(), "release");
        assert_matches!(cargo.subcommands[0].flags[0].description, Some(_));
        assert_matches!(cargo.subcommands[0].flags[0].short, Some(_));
        assert_matches!(cargo.subcommands[1].flags[0].long.as_str(), "release");
        assert_matches!(cargo.subcommands[1].flags[0].description, Some(_));
        assert_matches!(cargo.subcommands[1].flags[0].short, Some(_));
    }
}
