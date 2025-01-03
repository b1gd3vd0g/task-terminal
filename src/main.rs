use std::env;
use postgres::types::Timestamp;
use task_terminal::todo::Todo;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        let timestamp = Timestamp::
    }
}
