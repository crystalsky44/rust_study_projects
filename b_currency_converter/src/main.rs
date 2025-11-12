use std::env;
use std::process;
use b_currency_converter::{user_input_check, run};

fn main() {
    let user_input = user_input_check(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing your argument! \n{error}");
        process::exit(1);
    });

    run(user_input);
}

