use crate::db::pg_client;
use postgres::{Client, Error, Row};

/// The simplest of tasks - contains only a string representation
/// of the task to be completed.
pub struct Todo {
    task: String,
}

impl Todo {
    /// Creates a new Todo with a single task.
    ///
    /// # Examples
    ///
    /// ```
    /// use task_terminal::todo::Todo;
    /// let todo: Todo = Todo::new("Do this!".to_string());
    /// ```
    pub fn new(task: String) -> Todo {
        Todo { task }
    }

    /// Fetches all Todos from the database.
    ///
    /// # Examples
    ///
    /// ```
    /// // prints the `task` field from all todos.
    /// use task_terminal::todo::Todo;
    /// let rows = Todo::fetch_all().unwrap();
    /// for row in rows {
    ///     // 0 is the index of the `task` column.
    ///     println!("{}", row.get(0));
    /// }
    ///
    /// ```
    pub fn fetch_all() -> Result<Vec<Row>, Error> {
        let client: Result<Client, Error> = pg_client();
        let mut client: Client = match client {
            Ok(c) => c,
            Err(e) => {
                println!("{:?}", e);
                return Err(e);
            }
        };
        let rows: Result<Vec<Row>, Error> = client.query("SELECT * FROM todos", &[]);
        let rows: Vec<Row> = match rows {
            Ok(v) => v,
            Err(e) => {
                println!("{:?}", e);
                return Err(e);
            }
        };
        //println!("{:?}", rows);
        Ok(rows)
    }
    
    /// Saves the Todo to the database!
    ///
    /// # Examples
    ///
    /// ```
    /// use task_terminal::todo::Todo;
    /// let todo = Todo::new("Do this!".to_string());
    /// let rows_updated = todo.save();
    /// let rows_updated = match (rows_updated) {
    ///     Ok(n) => n,
    ///     Err(e) => {
    ///         println!("{:?}", e);
    ///         0
    ///     }
    /// };
    /// if rows_updated == 0 {
    ///     // failed
    /// } else {
    ///     // succeeded
    /// }
    /// ```
    pub fn save(&self) -> Result<u64, Error> {
        let client = pg_client();
        let mut client = match client {
            Ok(c) => c,
            Err(e) => {
                println!("{:?}", e);
                return Err(e);
            }
        };
        let rows_updated = client.execute("INSERT INTO todos VALUES ($1);", &[&self.task]);
        let rows_updated = match rows_updated {
            Ok(n) => n,
            Err(e) => {
                println!("{:?}", e);
                return Err(e);
            }
        };
        println!("Rows updated: {rows_updated}");
        Ok(rows_updated)
    }
}
