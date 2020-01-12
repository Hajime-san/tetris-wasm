use rand::{thread_rng, Rng};
use std::convert::TryInto;

use crate::store;

pub type FIELD = Vec<i32>;

#[derive(Debug, Default)]
pub struct Field(FIELD);

pub const FIELD_LENGTH: i32 = store::statics::Number::ROW * store::statics::Number::COLUMN;

impl Field {
    pub fn init() -> FIELD {
        let mut a = Vec::with_capacity(FIELD_LENGTH.try_into().unwrap());
        a = vec![0; FIELD_LENGTH.try_into().unwrap()];
        return a;
    }
}

#[cfg(test)]
mod tests {
    use super::store;
    use super::Field;
    use rand::{thread_rng, Rng};

    #[test]
    fn some_test() {
        let mut field = Field::init();
        field[0] = 999;
        println!("{:?}", &field);

        // let choices = [1, 2, 4, 8, 16, 32];
        // let mut rng = thread_rng();
        // println!("{:?}", rng.choose(&choices));
    }
}

pub fn main() {}
