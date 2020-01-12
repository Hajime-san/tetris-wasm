use std::iter::IntoIterator;

use crate::func;
use crate::store;

pub struct Movable {}

impl Movable {
    pub fn left(
        field: &store::dynamics::FIELD,
        current_block: &store::statics::BlockPosition,
    ) -> bool {
        let mut flag = false;
        for (i, v) in field.iter().enumerate() {
            for n in current_block {
                if i == *n as usize {
                    println!("{}", v);
                    flag = true;
                }
            }
        }

        for v in current_block {
            if func::fix_digit(*v) == func::fix_digit(store::statics::Number::ROW) {
                println!("{} is left side", *v);
                flag = true;
            }
        }

        return flag;
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
