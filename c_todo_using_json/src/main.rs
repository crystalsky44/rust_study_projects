#![warn(clippy::pedantic)]

mod lib;

use std::env;
use c_todo_using_json::Task;
use anyhow::Result;

// let's try implementing below main advised from ChatGPT
// make sure ask "why" the main might be more idiomatic than one above
fn main() {
    match run() {
        Ok(_) => println!("Task created successfully!"),
        Err(_) => println!("some error")
    }
}

fn run() -> Result<()> {
    let title = env::args().nth(1).ok_or(anyhow::Error)?;
    let description = get_user_input("Input description");

    let task = Task::new(title, description);

    task.store(path);

    println!("Created task: {task:?}");
    Ok(())
}

fn get_user_input(field: &str) -> String {
    print!("{field} ");

    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim_end().to_string()
}














// dump of old codes below, delete it when I'm sure I'm never coming back to it

/*
fn main() {
    Task::new(env::args().nth(1).unwrap());

    println!("Task loaded!");
}
*/

/*
fn app() -> /* some kind of Result (maybe use thiserror)*/{
    // some code
}
*/

/*
mod another_way {
    fn main() {
        // one thing I still wondering, is what do I put in main?
        let arg = env::args().nth(1).unwrap();
        // ideal for above probably is let arg = env::arg().nth(1)?;

        let task: Task = Task::new(arg); // this create a "Task" data,
                                         // which is NOT like the one above line

        append_task(task); // this appends task to file, but this also is
                           // different than the two lines above
    }
}
*/
