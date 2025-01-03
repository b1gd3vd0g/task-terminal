use postgres::{Client, Error, NoTls};

pub fn pg_client() -> Result<Client, Error> {
    Client::connect(
        "host=localhost user=b1gd3vd0g dbname=task_terminal",
        NoTls,
    )
}
