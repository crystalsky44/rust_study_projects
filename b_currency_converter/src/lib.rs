use std::ops::Mul;

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

