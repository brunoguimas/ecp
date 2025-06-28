/// Represents a CLI argument broken into simple strings.
///
/// This struct is the output of `App::run()` or `App::parse_args()`, providing a simple
/// way to access the components of a user's CLI input.
///
/// The `App::run()` method is the recommended way to parse CLI arguments, it
/// uses `std::env::args()` as the CLI input, which removes the necessity to
/// manually handle user input.
///
/// # Example:
/// ```ignore
/// use ecp::builder::*;
///
/// let cli = App::new("Rust")
///     .description("Rust programming language")
///     .version("0.1.0")
///     .command(
///         Command::new("cargo")
///             .description("Rust's package manager")
///             .subcommand(
///                 Command::new("build")
///                     .description("Compile the current package")
///                     .flag(
///                         Flag::new("release")
///                             .description("Build artifacts in release mode, with optimizations")
///                             .short('r'),
///                     )
///                     .flag(
///                         Flag::new("locked")
///                             .description("Assert that `Cargo.lock` will remain unchanged"),
///                     ),
///             )
///             .subcommand(
///                 Command::new("run")
///                     .description("Run a binary or example of the local package")
///                     .flag(
///                         Flag::new("release")
///                             .description("Build artifacts in release mode, with optimizations")
///                             .short('r'),
///                     )
///                     .flag(
///                         Flag::new("locked")
///                             .description("Assert that `Cargo.lock` will remain unchanged"),
///                     ),
///             ),
///     )
///     .run(); // Builds the CommandParsed
///
/// assert_eq!(cli.get_command(), "cargo");
/// assert_eq!(cli.get_subcommand(), Some("run"));
/// assert_eq!(cli.get_flags().any(|f| f == "release"), true);
/// assert_eq!(cli.get_flags().any(|f| f == "locked"), true);
/// assert_eq!(cli.get_values().any(|f| f == "port"), true);
/// assert_eq!(cli.get_values().any(|f| f == "8080"), true);
/// ```
pub struct CommandParsed {
    /// Represents the main command in the CLI argument.
    ///
    /// # Example:
    /// In `cargo run`, the command is `"cargo"`.
    pub(crate) command: String,

    /// Represents the optional subcommand in the CLI argument.
    ///
    /// # Example:
    /// In `cargo run`, the subcommand is Some("run").
    pub(crate) subcommand: Option<String>,

    /// Represents multiple flags present in the CLI argument.
    ///
    /// Flags can be short flags or long flags.
    /// Short flags starts with `-` and long flags starts with `--`.
    ///
    /// Example:
    /// In `cargo run --release`, the flags are ["release"].
    /// In `cargo build -p`, the flags are ["p"].
    pub(crate) flags: Vec<String>,

    /// Represents multiple values present in the CLI argument.
    /// Values are usually non-flags strings, often arguments to flags
    ///
    /// # Example:
    /// In `cargo run --bin my_binary`, the values are ["my_binary"].
    pub(crate) values: Vec<String>,
}

impl CommandParsed {
    /// Returns the main command in the CLI input.
    ///
    /// # Example:
    ///
    /// For `cargo run`, returns `"cargo"`.
    pub fn get_command(&self) -> &str {
        &self.command
    }

    /// Returns the subcommand of the CLI input.
    ///
    /// # Example:
    ///
    /// For `cargo run`, returns `Some("run")`.
    /// For `cargo`, returns `"None"`.   
    pub fn get_subcommand(&self) -> Option<&str> {
        self.subcommand.as_deref()
    }

    /// Returns an iterator over the flags in the CLI input.
    ///
    /// Flags are arguments that start with `-` or `--`.
    ///
    /// # Example:
    ///
    /// For `cargo build -p some_pkg --release`, this returns an iterator over:
    /// ["-p", "--release"]
    pub fn get_flags(&self) -> impl Iterator<Item = &str> {
        self.flags.iter().map(|s| s.as_str())
    }

    /// Returns an iterator over the values in the CLI input.
    ///
    /// Values are usually non-flags strings, often arguments to flags
    ///
    /// # Example:
    ///
    /// For `cargo run --bin my_binary`, this returns an iterator over:
    /// ["my_binary"]
    pub fn get_values(&self) -> impl Iterator<Item = &str> {
        self.values.iter().map(|s| s.as_str())
    }
}
