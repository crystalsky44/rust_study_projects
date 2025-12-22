use serde::{Serialize, Deserialize};

use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufWriter, BufReader, Write};
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    // id: u32, (this probably is the toughest... How do I keep track?)
    pub title: String,
    description: String,
    // status: Status,
    // created: String, // is date a String type?
    // due: Option<String>,
}

impl Task {
    pub fn new(task_title: String) -> Self {
        let task = task_input(task_title);
        let task_path = "task.json"; // for now, appneding to a specific file

        match File::create_new(task_path) {
            Ok(_) => write_task_to_file(task_path, &vec![task]),
            Err(_) => append_task(task_path, task),
        }

    }
}

fn append_task(path: impl AsRef<Path>, task: Task) {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    let mut task_vec: Vec<Task> = serde_json::from_reader(reader).unwrap();
    task_vec.push(task);

    write_task_to_file(&path, &task_vec);
}

fn write_task_to_file(path: impl AsRef<Path>, task_vec: &Vec<Task>) {
    let json_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    let writer = BufWriter::new(json_file);
    serde_json::to_writer_pretty(writer, &task_vec).unwrap();
}

fn task_input(title: String) -> Task {
    let description = input("input a description:");

    Task {
        title,
        description,
    }
}

fn input(field: &str) -> String {
    print!("{field} ");

    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim_end().to_string()
}

#[derive(Deserialize, Serialize, Debug)]
enum Status {
    ToDo,
    // InWork,
    // Done,
    // Pending,
}
