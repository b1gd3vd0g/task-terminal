use postgres::{Client, Error, NoTls};
use std::env;

pub fn pg_client() -> Result<Client, Error> {
    let config = env::var("PGCONFIG");
    let config = match config {
        Ok(c) => c,
        Err(_) => {
            println!("environment variable `PGCONFIG` not found!");
            "".to_string()
        }
    };
    Client::connect(&config.as_str(), NoTls)
}
