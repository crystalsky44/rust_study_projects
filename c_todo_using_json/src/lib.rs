use std::fs::File;

use serde::Serialize;
use serde_json::ser::Formatter;

#[derive(Serialize)]
pub struct Task {
    // id: u32, (this probably is the toughest... How do I keep track?)
    pub title: String,
    // description: Option<String>,
    status: Status,
    // created: String, // is date a String type?
    // due: Option<String>,
}

impl Task {
    pub fn new(task_title: String) -> Self {
        // fields "id", "title", "status", "created" cannot be
        // empty
        let mut task = Task {
            title: task_title,
            status: Status::ToDo, 
        };

        let mut file = File::create("test.json").unwrap();
        
        task.begin_array(&mut file).unwrap();

        serde_json::ser::to_writer_pretty(&file, &task).unwrap();

        task.end_array(&mut file).unwrap();

        task
    }
}

impl serde_json::ser::Formatter for Task {}

#[derive(Serialize)]
enum Status {
    ToDo,
    // InWork,
    // Done,
    // Pending,
}
