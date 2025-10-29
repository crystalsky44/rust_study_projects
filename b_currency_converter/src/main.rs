use std::io;
use std::io::Write;

struct UserMoney(f32);

impl UserMoney {
    fn truncate_after_hundreth(&mut self) {
        *self = (*self * 100.0).trunc() / 100.0;
    }
}

fn main() {

    let usd_eur_rate: f32 = 0.86;
    let mut input = String::new();

    user_input(&mut input);

    let mut user_money = UserMoney(input.trim_end().parse::<f32>().unwrap());
    // truncate_after_hundreth(&mut user_money);

    let mut result = UserMoney(user_money.truncate_after_hundreth() * usd_eur_rate);
    result.truncate_after_hundreth();
    // truncate_after_hundreth(&mut result);

    print!("your input: {result:?}");

}

fn user_input(input: &mut String) {
    print!("input a number! ");

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(input)
        .unwrap();
}
