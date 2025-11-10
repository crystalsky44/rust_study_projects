use std::env;
use std::collections::HashMap;
use b_currency_converter::UserMoney;

fn main() {
    let user_input = user_input_check(
        env::args()
    );

    run(user_input);
}

fn run(user_money: UserMoney) /*-> Result<(), Error>*/ {

    let converted = converter(user_money);

    print!("Converted!\nYou currently have: {}", converted.0);
}

fn user_input_check(mut args: impl Iterator<Item = String>,
    ) -> UserMoney {
    args.next();

    let Some(user_input) = args.next() else { 
        println!("No Entry!");
        panic!();
    };

    match user_input.parse::<f32>() {
        Ok(number) if number < 1_000_000.0 => UserMoney(number),
        Ok(_) => {
            println!("Too Big!");
            panic!();
        },
        Err(_) => { 
            println!("Not a Number!");
            panic!();
        },
    }
}

fn get_rate() -> f32 {
    let rates = HashMap::from([
       ("usd_eur", 0.86),
        ("usd_jpy", 150.0),
        ("eur_jpy", 175.0),
    ]);

    let subject_rate = "usd_eur";

    *rates.get(&subject_rate).unwrap()
}

fn converter(mut user_money: UserMoney) -> UserMoney {
    user_money.truncate_after_hundreth();
    let rate = get_rate();

    let mut converted = user_money * rate;
    converted.truncate_after_hundreth();

    converted
}
