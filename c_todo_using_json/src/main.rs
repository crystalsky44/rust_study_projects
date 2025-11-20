use std::fs;

fn main() {
    let _ = create_file();
}

fn create_file() -> std::io::Result<()> {
    let test_json_string = r#"
        {
            "tasks" : [
                {
                    "id" : 1,
                    "title" : "task_1",
                    "description" : "description of this task",
                    "status" : "todo",
                    "due_date" : "Nov-30-2025"
                },
                {
                    "id" : 2,
                    "title" : "task_2",
                    "description" : "description of this task",
                    "status" : "todo",
                    "due_date" : "Dec-11-2025"
                }
            ]
        }"#;
    fs::write("test.json", test_json_string)?;
    fs::write("bar.txt", "hello world without b")?;
    Ok(())
}
