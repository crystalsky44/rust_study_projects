use std::fs::{File, OpenOptions};

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

        match File::create_new("test.json") {
            Ok(mut json_file) => {
                task.begin_array(&mut json_file).unwrap();
                serde_json::ser::to_writer_pretty(&json_file, &task).unwrap();
                task.end_array(&mut json_file).unwrap();
            }  

            Err(_) => {
                let mut json_file = OpenOptions::new()
                    .append(true)
                    .open("test.json")
                    .unwrap();

                task.begin_array_value(&mut json_file, false).unwrap();
                serde_json::ser::to_writer_pretty(&json_file, &task).unwrap();
                task.end_array_value(&mut json_file).unwrap();
            }
        }

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
