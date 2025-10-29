use std::io;
use std::io::Write;

#[derive(Debug)]
struct UserMoney(f32);

impl UserMoney {
    fn truncate_after_hundreth(&mut self) {
        self.0 = (self.0 * 100.0).trunc() / 100.0;
    }
}

impl std::ops::Mul<f32> for UserMoney {
    type Output = UserMoney;

    fn mul(self, rhs: f32) -> UserMoney {
        UserMoney(self.0 * rhs)
    }
}

fn main() {

    let usd_eur_rate: f32 = 0.86;
    let mut input = String::new();

    user_input(&mut input);

    let mut user_money = UserMoney(input.trim_end().parse::<f32>().unwrap());
    user_money.truncate_after_hundreth();

    let mut converted = user_money * usd_eur_rate;
    converted.truncate_after_hundreth();

    print!("Converted!\nYou currently have: {}", converted.0);

}

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
