#[path = "controll.rs"]
mod controll;
#[path = "data.rs"]
mod data;
#[path = "func.rs"]
mod func;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::iter::IntoIterator;

pub struct Movable {
    _flag: bool,
}

impl Movable {
    pub fn left(field_array: &data::Field, current_block: &data::BlockPosition) -> bool {
        let mut flag = false;
        for (i, v) in field_array.iter().enumerate() {
            for n in current_block {
                if i == *n as usize {
                    println!("{}", v);
                    flag = true;
                }
            }
        }

        for v in current_block {
            if func::fix_digit(*v) == func::fix_digit(data::Number::ROW) {
                println!("{} is left side", *v);
                flag = true;
            }
        }

        return flag;
    }
}

fn main() {
    let mut init_field = controll::Field::init();

    // let collection = [5, 6, 14, 10];

    // let test = Movable::left(&init_field, &collection);

    // init_field[5] = 9;
    // init_field[6] = 99;
    // init_field[14] = 999;
    // init_field[15] = 9999;

    // println!("{:?}", init_field);

    // init_field[5] = 8888;

    // println!("{:?}", init_field);

    // let choices = [1, 2, 4, 8, 16, 32];
    // let mut rng = thread_rng();
    // println!("{:?}", choices.choose(&mut rng));
}
