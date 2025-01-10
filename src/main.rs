use dotenv::dotenv;
use std::{env, process};
use todo;

fn main() {
    // configure the environment variable.
    let config = dotenv();
    match config {
        Ok(_) => (),
        Err(e) => {
            println!("{:?}", e);
            panic!("Could not configure environment variable!");
        }
    }
    let args: Vec<String> = env::args().collect();
    todo::run(args);
}
