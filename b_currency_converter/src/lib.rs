use std::ops::Mul;
use std::collections::HashMap;

pub struct UserMoney(pub f32);

impl UserMoney {
   pub fn truncate_after_hundreth(&mut self) {
        self.0 = (self.0 * 100.0).trunc() / 100.0;
    }
}

impl Mul<f32> for UserMoney {
    type Output = UserMoney;

    fn mul(self, rhs: f32) -> UserMoney {
        UserMoney(self.0 * rhs)
    }
}

pub fn run(user_money: UserMoney) /*-> Result<(), Error>*/ {

    let converted = converter(user_money);

    print!("Converted!\nYou currently have: {}", converted.0);
}

/// # Errors
///
/// Will return a &str type, which will be printed at main
pub fn user_input_check(mut args: impl Iterator<Item = String>,
    ) -> Result<UserMoney, &'static str> {
    args.next();

    let Some(user_input) = args.next() else { 
        return Err("No Entry...")
    };

    match user_input.parse::<f32>() {
        Ok(number) if number < 1_000_000.0 => Ok(UserMoney(number)),
        Ok(_) => {
            Err("Too big sir...")
        },
        Err(_) => { 
            Err("Not a Number...")
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
