#![warn(clippy::pedantic)]

use serde::{Serialize, Deserialize};
use anyhow::Result;

use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufWriter, BufReader, Write};
use std::path::Path;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Task {
    // id: u32, (this probably is the toughest... How do I keep track?)
    pub title: String,
    description: String,
    // status: Status,
    // created: String, // is date a String type?
    // due: Option<String>,
}

impl Task {
    pub fn new(title: String) -> Task {
        let description = get_user_input("input a description:");

        Task {
            title,
            description,
        }
    }

    #[cfg(any())]
    pub fn store(self, storage: impl AsRef<Path>) -> Result<()> { 
        let writer = BufWriter::new(storage);
        serde_json::to_writer_pretty(writer, &vec![self])?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
enum Status {
    ToDo,
    // InWork,
    // Done,
    // Pending,
}

#[cfg(any())]
fn check_for_storage() {
    match File::create_new(&task_path) {
        Ok(_) => write_task_to_file(task_path, &vec![self])?,
        Err(_) => None,
    }
}

#[cfg(any())]
fn append_task(path: impl AsRef<Path>, task: Task) -> Result<()> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut task_vec: Vec<Task> = serde_json::from_reader(reader)?;
    task_vec.push(task);

    write_task_to_file(&path, &task_vec)
}

#[cfg(any())]
fn write_task_to_file(path: impl AsRef<Path>, task_vec: &Vec<Task>) -> Result<()> {
    let json_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;

    let writer = BufWriter::new(json_file);
    serde_json::to_writer_pretty(writer, &task_vec)?;
    Ok(())
}

fn get_user_input(field: &str) -> String {
    print!("{field} ");

    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

        #[test]
        fn add_task() {
            let title = get_user_input("input task title: ");
            let task = Task::new(title);

            let result = Task {
                title: "task1".to_string(),
                description: "some description".to_string(),
            };

            assert_eq!(task, result);
        }
}
