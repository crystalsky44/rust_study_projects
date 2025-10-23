use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();

    loop {
        print!("Input a number to get a factorial! ");
        io::stdout()
            .flush()
            .unwrap();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        match input.trim_end().parse::<u128>() {
            Ok(1..35) => break,
            _ => {
                input.clear();
                println!("[Invalid input]\nOnly supports a natural number between 1 and 34 due to memory overflow...");
            },
        }
    }

    let input = input.trim_end().parse::<u128>().unwrap();
    let mut product: u128 = input;

    for n in (1..input).rev() {
        product *= n;
    }

    print!("factorial of {input} ({input}!) is: {product}");
}
