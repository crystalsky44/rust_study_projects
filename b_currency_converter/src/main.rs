use std::io;
use std::io::Write;

#[derive(Debug)]
struct UserMoney(f32);

impl UserMoney {
    fn truncate_after_hundreth(mut self) -> Self {
        self.0 = (self.0 * 100.0).trunc() / 100.0;
        self
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

    let user_money = UserMoney(input.trim_end().parse::<f32>().unwrap());
    // truncate_after_hundreth(&mut user_money);

    let converted = user_money.truncate_after_hundreth() * usd_eur_rate;
    let result = converted.truncate_after_hundreth();
    // truncate_after_hundreth(&mut result);

    print!("Converted!\nYou currently have: {}", result.0);

}

fn user_input(input: &mut String) {
    print!("input a number! ");

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(input)
        .unwrap();

    println!();
}
