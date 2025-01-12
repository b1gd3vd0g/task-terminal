use std::{env, process};

/// Provides a connection to the postgres database.
mod db;

/// Provides functionality for the command `todo help`.
pub mod help;

/// Provides functionality for the command `todo list`.
pub mod list;

/// Provides implementation of a Task.
pub mod task;

/// Provides helper functions for other modules.
mod util;

/// Based on the arguments provided by the user (`args`), execute the proper
/// function.
pub fn run(args: Vec<String>) {
    // There is no command.
    if args.len() < 2 {
        print_intro();
        process::exit(0);
    }
    // There is a command.
    let command = args[1].as_str();
    match command {
        "help" | "--help" => help::print_help(),
        "list" => list::execute(args),
        _ => print_intro(),
    }
}

/// Print the name of the app and version.
fn print_version() {
    let (name, version): (String, String) = (
        env::var("CARGO_PKG_NAME").unwrap_or(String::from("todo")),
        env::var("CARGO_PKG_VERSION").unwrap_or(String::from("?")),
    );
    println!("{name} v{version}");
}

/// Print the name of the app and version, as well as how to get help.
fn print_intro() {
    print_version();
    println!("For help, use --help.");
}
