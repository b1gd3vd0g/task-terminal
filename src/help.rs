/// Print the instructions for using the command line tool.
pub fn print_help() {
    super::print_version();
    println!("A command line productivity tool.\n");
    println!("use: todo <command> [<args>]\n");

    println!("For help with specific commands, use \"todo <command> --help\"\n");

    println!("COMMANDS:\n");
    print_to_len("help", 10);
    println!("View this screen");
    print_to_len("list", 10);
    println!("List all tasks currently in progress.");
    print_to_len("add", 10);
    println!("Add a new task.");
    print_to_len("check", 10);
    println!("Check off (complete) a task.");
}

fn print_to_len(output: &str, len: usize) {
    print!("{output}");
    let spaces = len - output.len();
    for _ in 0..spaces - 1 {
        print!(" ");
    }
}
