use std::iter::IntoIterator;

use crate::func;
use crate::store;

pub struct Movable {}

impl Movable {
    pub fn left(field: &Vec<i32>, current_block: &store::statics::BlockPosition) -> bool {
        let mut flag = false;

        // check fixed block
        for (i, _v) in field.iter().enumerate() {
            for w in current_block {
                if *w as usize == (i + store::statics::Number::RIGHT_MOVE as usize) {
                    flag = true;
                }
            }
        }

        // check side wall
        for v in current_block {
            if func::fix_digit(*v) == func::fix_digit(store::statics::Number::ROW) {
                println!("{} is left side", *v);
                flag = true;
            }
        }

        flag
    }

    pub fn right(field: &Vec<i32>, current_block: &store::statics::BlockPosition) -> bool {
        let mut flag = false;

        // check fixed block
        for (i, _v) in field.iter().enumerate() {
            for w in current_block {
                if *w as usize == (i + store::statics::Number::LEFT_MOVE as usize) {
                    flag = true;
                }
            }
        }

        // check side wall
        for v in current_block {
            if func::fix_digit(*v)
                == func::fix_digit(store::statics::Number::ROW + store::statics::Number::LEFT_MOVE)
            {
                println!("{} is left side", *v);
                flag = true;
            }
        }

        flag
    }

    pub fn down(field: &Vec<i32>, current_block: &store::statics::BlockPosition) -> bool {
        let mut flag = false;

        // check fixed block
        for (i, _v) in field.iter().enumerate() {
            for _ in current_block {
                if field[i - store::statics::Number::ROW as usize]
                    == store::statics::Number::CURRENT
                {
                    flag = true;
                }
            }
        }

        // check side wall
        for v in current_block {
            if func::fix_digit(*v)
                == func::fix_digit(store::statics::Number::ROW + store::statics::Number::LEFT_MOVE)
            {
                println!("{} is left side", *v);
                flag = true;
            }
        }

        flag
    }
}

#[cfg(test)]
mod tests {
    use super::store;
    use super::Movable;

    #[test]
    fn some_test() {}
}

fn main() {}
