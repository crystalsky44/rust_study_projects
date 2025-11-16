use std::env;
use b_currency_converter::{user_input_check, run};

fn main() {
    if let Err(e) = app() {
        eprintln!("{}", e);
    }
}

fn app() -> Result<(), Box<dyn std::error::Error>> {
    let user_input = user_input_check(env::args())?;

    run(user_input);
    Ok(())
}
