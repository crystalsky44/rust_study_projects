use std::env;
use c_todo_using_json::Task;

fn main() {
    let task = Task::new(env::args().nth(1).unwrap());

    println!("Created task: {}", task.title);
}

/*
fn app() -> /* some kind of Result (maybe use thiserror)*/{
    // some code 
}
*/
