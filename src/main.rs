use dotenv::dotenv;
use task_terminal::todo::Todo;

fn main() {
    dotenv().ok();

    // TEMPORARY: gets all the todos from the db and prints the tasks.
    let rows = Todo::fetch_all();
    let rows = match rows {
        Ok(r) => r,
        Err(e) => {
            println!("{:?}", e);
            panic!("Error fetching all rows!");
        }
    };
    for row in rows {
        let task: String = row.get(0);
        println!("{}", task);
    }
}
