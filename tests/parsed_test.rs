use ecp::builder::{App, Command, Flag};

#[test]
fn full_parsed() {
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

    assert_eq!(cli.get_command(), "cargo");
    assert_eq!(cli.get_subcommand(), Some("run"));
    assert_eq!(cli.get_flags().any(|f| f == "release"), true);
    assert_eq!(cli.get_flags().any(|f| f == "locked"), true);
    assert_eq!(cli.get_values().any(|f| f == "port"), true);
    assert_eq!(cli.get_values().any(|f| f == "8080"), true);
}
