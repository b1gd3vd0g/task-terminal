use dotenv::dotenv;
use std::env;
use bdd_todo as todo;

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
