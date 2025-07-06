use std::env;
use std::ffi;

use crate::builder::Command;
use crate::errors::AppError;
use crate::parser::CommandParsed;
use crate::parser::utils::*;

/// Contains all commands of the CLI app.
///
/// This struct can be parsed into a [`CommandParsed`] using multiple parse methods, the most
/// recommended of them is [`App::run()`], which uses [`std::env::args_os()`] as the CLI input,
/// removing the need to manually handle user input.
/// You can also use [`App::try_run()`] to manually handle errors that may occur during parsing.
/// You can use [`App::parse_args()`] or [`App::try_parse_args()`] to manually handle user input.

pub struct App {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) commands: Vec<Command>,
}

impl App {
    // Setters

    /// Creates a new [`App`] with a unique name.
    pub fn new(name: &str) -> App {
        App {
            name: name.to_string(),
            version: None,
            description: None,
            commands: Vec::new(),
        }
    }

    /// Sets the version.
    pub fn version(mut self, version: &str) -> App {
        self.version = Some(version.to_string());
        self
    }

    /// Sets the description.
    pub fn description(mut self, description: &str) -> App {
        self.description = Some(description.to_string());
        self
    }

    /// Adds a command to the [`App`].
    ///
    /// # Example:
    /// ```
    /// use ecp::builder::*;
    ///
    /// let app = App::new("Rust")
    ///             .command(
    ///                 Command::new("cargo")
    ///             );
    /// ```
    pub fn command(mut self, command: Command) -> App {
        self.commands.push(command);
        self
    }
    /// Attempts to returns a [`CommandParsed`] containing the user input broken into strings to
    /// simpler use.
    /// If you don't want to handle user input manually use [`App::run()`] method.
    /// If you want to handle user input without manual handling errors use [`App::parse_args()`]
    ///
    /// # Errors:
    /// This function will return an [`AppError`] if the args don't match the [`App`] fields.
    /// See the [`AppError`] struct for more info.
    ///
    /// # Example:
    /// ```
    /// use std::ffi;
    /// use ecp::builder::*;
    ///
    /// let args: Vec<ffi::OsString> = vec![
    ///     "ecp".into(),
    ///     "cargo".into(),
    ///     "run".into(),
    ///     "-r".into(),
    ///     "--locked".into(),
    ///     "port".into(),
    ///     "8080".into(),
    /// ];
    ///
    /// let cli = App::new("Rust")
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
    ///     .try_parse_args(args);
    ///     
    /// let app = match cli {
    ///     Ok(parsed) => parsed,
    ///     Err(error) => {
    ///         eprintln!("{}", error);
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// assert_eq!(app.get_command(), "cargo");
    /// assert_eq!(app.get_subcommand(), Some("run"));
    /// assert_eq!(app.get_flags().any(|f| f == "release"), true);
    /// assert_eq!(app.get_flags().any(|f| f == "locked"), true);
    /// assert_eq!(app.get_values().any(|f| f == "port"), true);
    /// assert_eq!(app.get_values().any(|f| f == "8080"), true);
    /// ```
    pub fn try_parse_args(&self, args: Vec<ffi::OsString>) -> Result<CommandParsed, AppError> {
        let args_utf8: Vec<String> = args
            .iter()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect();

        if args_utf8.len() < 2 {
            return Err(AppError::InvalidInput("Too few arguments".to_string()));
        }

        let command = get_command(&self, &args_utf8)?;
        let subcommand = get_subcommand(&self, &args_utf8)?;
        let flags = get_flags(&self, &args_utf8)?;
        let values = get_values(&args_utf8, &subcommand, &flags)?;

        Ok(CommandParsed {
            command,
            subcommand,
            flags,
            values,
        })
    }

    /// Returns a [`CommandParsed`] containing the user input broken into strings to simpler use.
    /// If you don't want to handle user input manually use [`App::run()`]
    /// If you want to manually handle errors use the method [`App::try_parse_args()`]
    ///
    /// # Errors:
    /// This function will print a error if the args don't match the [`App`] fields.
    /// See the [`AppError`] struct for more info.
    ///
    /// # Example:
    /// ```
    /// use std::ffi;
    /// use ecp::builder::*;
    ///
    /// let args: Vec<ffi::OsString> = vec![
    ///     "ecp".into(),
    ///     "cargo".into(),
    ///     "run".into(),
    ///     "-r".into(),
    ///     "--locked".into(),
    ///     "port".into(),
    ///     "8080".into(),
    /// ];
    ///
    /// let app = App::new("Rust")
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
    ///     .parse_args(args);
    ///     
    /// assert_eq!(app.get_command(), "cargo");
    /// assert_eq!(app.get_subcommand(), Some("run"));
    /// assert_eq!(app.get_flags().any(|f| f == "release"), true);
    /// assert_eq!(app.get_flags().any(|f| f == "locked"), true);
    /// assert_eq!(app.get_values().any(|f| f == "port"), true);
    /// assert_eq!(app.get_values().any(|f| f == "8080"), true);
    /// ```
    pub fn parse_args(&self, args: Vec<ffi::OsString>) -> CommandParsed {
        match self.try_parse_args(args) {
            Ok(parsed) => parsed,
            Err(e) => e.exit(),
        }
    }

    /// Returns a [`CommandParsed`] containing user input broken into strings to simpler use.
    /// This function automatically uses std::env::args_os() as the user input.
    /// If you want to handle user input manually use [`App::parse()`] or [`App::try_parse()`].
    /// If you don't want to handle errors manually use [`App::try_run()`].
    ///
    /// # Errors:
    /// This function will print a error if the args don't match the [`App`] fields.
    /// See the [`AppError`] struct for more info.
    ///
    /// # Example:
    /// ```
    /// use ecp::builder::*;
    ///
    /// let cli = App::new("Rust")
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
    ///     .try_parse_args(args);
    ///      
    /// assert_eq!(app.get_command(), "cargo");
    /// assert_eq!(app.get_subcommand(), Some("run"));
    /// assert_eq!(app.get_flags().any(|f| f == "release"), true);
    /// assert_eq!(app.get_flags().any(|f| f == "locked"), true);
    /// assert_eq!(app.get_values().any(|f| f == "port"), true);
    /// assert_eq!(app.get_values().any(|f| f == "8080"), true);
    /// ```
    pub fn run(&self) -> CommandParsed {
        let args: Vec<ffi::OsString> = env::args_os().collect();

        match self.try_parse_args(args) {
            Ok(parsed) => parsed,
            Err(e) => e.exit(),
        }
    }
    /// Returns a [`CommandParsed`] containing user input broken into strings to simpler use.
    /// This function automatically uses std::env::args_os() as the user input.
    /// Use this method if you want to manually handle errors.
    ///
    /// # Errors:
    /// This function will print a error if the args don't match the [`App`] fields.
    /// See the [`AppError`] struct for more info.
    ///
    /// # Example:
    /// ```
    /// use ecp::builder::*;
    ///
    /// let cli = App::new("Rust")
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
    ///     .try_parse_args(args);
    ///      
    /// let app = match cli {
    ///     Ok(parsed) => parsed,
    ///     Err(error) => {
    ///         eprintln!("{}", error);
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// assert_eq!(app.get_command(), "cargo");
    /// assert_eq!(app.get_subcommand(), Some("run"));
    /// assert_eq!(app.get_flags().any(|f| f == "release"), true);
    /// assert_eq!(app.get_flags().any(|f| f == "locked"), true);
    /// assert_eq!(app.get_values().any(|f| f == "port"), true);
    /// assert_eq!(app.get_values().any(|f| f == "8080"), true);
    /// ```

    pub fn try_run(&self) -> Result<CommandParsed, AppError> {
        let args: Vec<ffi::OsString> = env::args_os().collect();

        self.try_parse_args(args)
    }
    /// Returns the app name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    // Getters

    /// Returns the app version.
    pub fn get_version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns the app description.
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns an iterator over the commands in the app.
    pub fn get_commands(&self) -> impl Iterator<Item = &Command> {
        self.commands.iter()
    }
}
