use postgres::{Client, Error, NoTls};

pub fn pg_client() -> Result<Client, Error> {
    Client::connect(
        "host=localhost user=b1gd3vd0g password=JHBPmttydP5046! dbname=task_terminal",
        NoTls,
    )
}
