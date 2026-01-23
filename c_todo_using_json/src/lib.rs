#![warn(clippy::pedantic)]

use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::{BufWriter, BufReader, Write, ErrorKind};
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
    pub fn new(title: String, description: String) -> Task {
        Task {
            title,
            description,
        }
    }

    #[cfg(any())]
    pub fn store(self, path: impl AsRef<Path>) -> Result<()> { 
        let path = path.as_ref();

        let task_vec = load_task_vec(path);
        task_vec.push(self);

        write_task_vec(path);

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

fn load_task_vec<P: AsRef<Path>>(path: P) -> Result<Vec<Task>> {
    match fs::read_to_string(path) {
        Ok(json_string) => {
            if json_string.is_empty() {
                Ok(Vec::new()) // can the compiler infer Vec<Task> here?
            } else {
                let task_vec = serde_json::from_str(&json_string)
                    .with_context(|| format!("failed to parse JSON in {}", path.display()))?;
                Ok(task_vec)
            }
        },
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error)
            .with_context(|| format!("failed to read {}", path.display())),
    }
}

#[cfg(any())]
fn write_to_json_file(path: impl AsRef<Path>, task_vec: &Vec<Task>) -> Result<()> {
    /* not going to implement it now
    if let Some(parent_directory) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create dir {}", parent.display()))?;
    }
    */

    let json_array = serde_json::to_string_pretty(task_vec)
        .context("failed to serialize Task vector to JSON")?;

    let tmp = path.with_extension("tmp");

    fs::write(&tmp, json_array)
        .with_context(|| format!("failed to write temp file {}", tmp.display()))?;

    fs::rename(&tmp, path)
        .with_context(|| format!("failed to rename {} -> {}", tmp.display(), path.display()))?;

    Ok(())
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
        #[cfg(any())]
        fn check_file() {
            let title = String::from("task-1");
            let task = Task::new(title);
            let path = "task.json";

            // test logic from here

            task.store(&path);

            let file = File::open(&path).unwrap();
            let reader = BufReader::new(file);
            let task_vec: Vec<Task> = serde_json::from_reader(reader).unwrap();

            println!("{task_vec:?}");
        }

        #[test]
        fn load_task() {
            let vec_task = load_task_vec("task.json").unwrap();
            println!("inside file: {vec_task}");
        }
}
