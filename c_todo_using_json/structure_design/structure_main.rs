use std::env;
use c_todo_using_json::Task;

fn main() {
    Task::new(env::args().nth(1).unwrap());

    println!("Task inserted!");
}

/*
fn app() -> /* some kind of Result (maybe use thiserror)*/{
    // some code 
}
*/
