#[path = "data.rs"]
mod data;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::iter::IntoIterator;

pub struct Movable {
    _flag: bool,
}

impl Movable {
    pub fn left(field_array: data::Field, current_block: data::BlockPosition) -> bool {
        let mut flag = false;
        for v in field_array.into_iter() {
            println!("{}", v); // x: i32
        }
        return flag;
    }
}

fn main() {

    // let choices = [1, 2, 4, 8, 16, 32];
    // let mut rng = thread_rng();
    // println!("{:?}", choices.choose(&mut rng));
}
