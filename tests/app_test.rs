use ecp::builder::{App, Command, Flag};

#[test]
fn full_app() {
    let args = vec![
        "ecp".to_string(),
        "cargo".to_string(),
        "run".to_string(),
        "-r".to_string(),
        "--locked".to_string(),
        "port".to_string(),
        "8080".to_string(),
    ];

    let cli = App::new("Rust")
        .command(
            Command::new("cargo")
                .description("Rust's package manager")
                .subcommand(
                    Command::new("build")
                        .description("Compile the current package")
                        .flag(
                            Flag::new("release")
                                .description("Build artifacts in release mode, with optimizations")
                                .short('r'),
                        )
                        .flag(
                            Flag::new("locked")
                                .description("Assert that `Cargo.lock` will remain unchanged"),
                        ),
                )
                .subcommand(
                    Command::new("run")
                        .description("Run a binary or example of the local package")
                        .flag(
                            Flag::new("release")
                                .description("Build artifacts in release mode, with optimizations")
                                .short('r'),
                        )
                        .flag(
                            Flag::new("locked")
                                .description("Assert that `Cargo.lock` will remain unchanged"),
                        ),
                ),
        )
        .parse_args(&args)
        .unwrap();

    assert_eq!(cli.command, "cargo");
    assert_eq!(cli.subcommand, Some("run".to_string()));
    assert_eq!(cli.flags[0], "release");
    assert_eq!(cli.flags[1], "locked");
    assert_eq!(cli.values[0], "port");
    assert_eq!(cli.values[1], "8080")
}
