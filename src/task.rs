use crate::db::pg_client;
use crate::util::system_time_to_date_time;
use chrono::DateTime;
use postgres::{Error, Row};
use std::time::SystemTime;

#[derive(Debug)]
pub struct Task {
    task: String,
    created: String,
    completed: bool,
    id: i32,
}

impl Task {
    pub fn from(row: Row) -> Task {
        let mut task: String = String::new();
        let mut created: String = String::from("2000-03-17");
        let mut completed: bool = false;
        let mut id: i32 = 0;

        let mut i = 0;
        for col in row.columns() {
            let name = col.name();
            match name {
                "task" => {
                    let value: String = row.get(i);
                    task = value;
                }
                "created" => {
                    let st: SystemTime = row.get(i);
                    let date_time = match system_time_to_date_time(st) {
                        Some(dt) => dt,
                        None => DateTime::from_timestamp(0, 0).unwrap(),
                    };
                    created = date_time.to_string();
                }
                "completed" => {
                    let value: bool = row.get(i);
                    completed = value;
                }
                "id" => {
                    let value: i32 = row.get(i);
                    id = value;
                }
                _ => {
                    panic!("Could not understand column name.")
                }
            }
            i += 1;
        }
        Task {
            task,
            created,
            completed,
            id,
        }
    }

    pub fn fetch_all_tasks() -> Result<Vec<Task>, Error> {
        let rows = match Task::fetch_all_rows() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        Ok(rows
            .iter()
            .map(|row| {
                let r = row.clone();
                Task::from(r)
            })
            .collect())
    }

    fn fetch_all_rows() -> Result<Vec<Row>, Error> {
        let mut client = match pg_client() {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        client.query("SELECT * FROM tasks;", &[])
    }

    pub fn task(&self) -> String {
        self.task.clone()
    }

    pub fn created(&self) -> String {
        self.created.clone()
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}
