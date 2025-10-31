// use std::io;
// use std::io::Write;
use std::ops::Mul;
use std::env;

struct UserMoney(f32);

impl UserMoney {
    fn truncate_after_hundreth(&mut self) {
        self.0 = (self.0 * 100.0).trunc() / 100.0;
    }
}

impl Mul<f32> for UserMoney {
    type Output = UserMoney;

    fn mul(self, rhs: f32) -> UserMoney {
        UserMoney(self.0 * rhs)
    }
}

fn main() {

    let usd_eur_rate: f32 = 0.86;
    // let mut input = String::new();

    // user_input(&mut input);

    let mut arg_input = env::args();
    arg_input.next();

    let Some(user_input) = arg_input.next() else { 
        println!("No Entry!");
        return 
    };

    let mut user_money = match user_input.parse::<f32>() {
        Ok(number) if number < 1_000_000.0 => UserMoney(number),
        Ok(_) => {
            println!("Too Big!");
            return
        },
        Err(_) => { 
            println!("Not a Number!");
            return 
        },
    };

    user_money.truncate_after_hundreth();

    let mut converted = user_money * usd_eur_rate;
    converted.truncate_after_hundreth();

    print!("Converted!\nYou currently have: {}", converted.0);

}

/*
fn user_input(input: &mut String) {
    print!("input a number: ");

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(input)
        .unwrap();

    println!();
}
*/
