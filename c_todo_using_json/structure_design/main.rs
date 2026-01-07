fn main() {
    match run() {
        Ok => println!("Task successfully saved!");
        Err => println!("something went wrong");
    }
}

fn run() -> Result<()> {
    let title = env::args(); // assign argument to title
    let task = Task::new(title);

    let storage = match check_for_storage().expect("something went wrong") {
        Some(storage) => storage,
        None => create_new_storage()?,
    };

    task.store(storage)?;

    Ok(())
}

