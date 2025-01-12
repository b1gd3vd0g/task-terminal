use crate::task::Task;

pub fn execute(_args: Vec<String>) {
    let tasks = match Task::fetch_all_tasks() {
        Ok(t) => t,
        Err(_) => panic!("Could not fetch all rows!"),
    };

    for task in tasks {
        task.print();
    }
}
