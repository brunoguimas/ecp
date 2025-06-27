use ecp::builder::*;

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
    assert_eq!(cargo.get_name(), "cargo");
    assert_eq!(
        cargo.get_description().as_deref(),
        Some("Rust's package manager")
    );

    // Subcommands
    assert_eq!(cargo.get_subcommands()[0].get_name().as_str(), "build");
    assert_eq!(
        cargo.get_subcommands()[0]
            .get_description()
            .as_ref()
            .unwrap(),
        "Compile the current package"
    );
    assert_eq!(cargo.get_subcommands()[1].get_name().as_str(), "run");
    assert_eq!(
        cargo.get_subcommands()[1]
            .get_description()
            .as_ref()
            .unwrap(),
        "Run a binary or example of the local package"
    );

    // Flags
    assert_eq!(
        cargo.get_subcommands()[0].get_flags()[0]
            .get_long()
            .as_str(),
        "release"
    );
    assert_eq!(
        cargo.get_subcommands()[0].get_flags()[0]
            .get_description()
            .as_ref()
            .unwrap(),
        "Build artifacts in release mode, with optimizations"
    );
    assert_eq!(
        cargo.get_subcommands()[0].get_flags()[0]
            .get_short()
            .unwrap(),
        'r'
    );
    assert_eq!(
        cargo.get_subcommands()[1].get_flags()[0]
            .get_long()
            .as_str(),
        "release"
    );
    assert_eq!(
        cargo.get_subcommands()[1].get_flags()[0]
            .get_description()
            .as_ref()
            .unwrap(),
        "Build artifacts in release mode, with optimizations"
    );
    assert_eq!(
        cargo.get_subcommands()[1].get_flags()[0]
            .get_short()
            .unwrap(),
        'r'
    );
}

#[test]
fn one_subcommand_build() {
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
    assert_eq!(cargo.get_name().as_str(), "cargo");
    assert_eq!(
        cargo.get_description().as_ref().unwrap(),
        "Rust's package manager"
    );

    // Subcommand
    assert_eq!(cargo.get_subcommands()[0].get_name().as_str(), "build");
    assert_eq!(
        cargo.get_subcommands()[0]
            .get_description()
            .as_ref()
            .unwrap(),
        "Compile the current package"
    );

    // Flag
    assert_eq!(
        cargo.get_subcommands()[0].get_flags()[0]
            .get_long()
            .as_str(),
        "release"
    );
    assert_eq!(
        cargo.get_subcommands()[0].get_flags()[0]
            .get_description()
            .as_ref()
            .unwrap(),
        "Build artifacts in release mode, with optimizations"
    );
    assert_eq!(
        cargo.get_subcommands()[0].get_flags()[0]
            .get_short()
            .unwrap(),
        'r'
    );
}
