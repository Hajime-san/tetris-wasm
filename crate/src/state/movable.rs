use crate::func;
use crate::store;

use store::dynamics::field::FieldContext;

pub fn left(field_collection: &FieldContext, current_block: &store::statics::BlockPosition) -> bool {
    let mut flag = true;

    let field = field_collection.get_list();

    // check fixed block
    for (i, v) in field.iter().enumerate() {
        let fixed_blocks = v != &store::statics::Number::CURRENT && v != &store::statics::Number::EMPTY;

        if fixed_blocks {

            for w in current_block {
                if (w + &store::statics::Number::LEFT_MOVE) == i as i32 {
                    flag = false;
                }
            }
        }
    }

    // check side wall
    for v in current_block {
        if func::fix_digit(*v)
            == func::fix_digit(store::statics::Number::ROW)
        {
            flag = false;
        }
    }

    flag
}

pub fn right(field_collection: &FieldContext, current_block: &store::statics::BlockPosition) -> bool {
    let mut flag = true;

    let field = field_collection.get_list();

    // check fixed block
    for (i, v) in field.iter().enumerate() {
        let fixed_blocks = v != &store::statics::Number::CURRENT && v != &store::statics::Number::EMPTY;

        if fixed_blocks {

            for w in current_block {
                if (w + &store::statics::Number::RIGHT_MOVE) == i as i32 {
                    flag = false;
                }
            }
        }
    }

    // check side wall
    for v in current_block {
        if func::fix_digit(*v)
            == func::fix_digit(store::statics::Number::ROW + store::statics::Number::LEFT_MOVE)
        {
            flag = false;
        }
    }

    flag
}

pub fn down(field_collection: &FieldContext, current_block: &store::statics::BlockPosition) -> bool {
    let mut flag = true;

    let field = field_collection.get_list();

    // check fixed block
    for (i, v) in field.iter().enumerate() {

        // pass runtime error, when access values that outs of vector length
        let index = i as i32 - store::statics::Number::ROW;
        if index < 0 {
            continue;
        }

        let fixed_blocks = v != &store::statics::Number::CURRENT && v != &store::statics::Number::EMPTY;


            if &field[index as usize] == &store::statics::Number::CURRENT && fixed_blocks {
                flag = false;
            }
    }

    // check last row
    for v in current_block {
        let is_last_row = field
                    .iter()
                    .any(|_| v >= &(field.len() as i32 - store::statics::Number::ROW));
        if is_last_row {
            flag = false;
        }
    }

    flag
}

pub fn rotate(field_collection: &FieldContext, tmp_block: &store::statics::BlockPosition) -> bool {
    let mut left_wall = true;
    let mut right_wall = true;
    let mut down_wall = true;

    let field = field_collection.get_list();

    // wall check down/left/right
    for v in tmp_block {
        let is_last_row = field
            .iter()
            .any(|_| v >= &(field.len() as i32));

        if is_last_row {
            down_wall = false;
        }
        if func::fix_digit(*v) == func::fix_digit(store::statics::Number::ROW) {
            left_wall = false;
        }
        if func::fix_digit(*v)
            == func::fix_digit(store::statics::Number::ROW + store::statics::Number::LEFT_MOVE)
        {
            right_wall = false;
        }
    }

    // fixed block check
    let mut is_filled = true;

    for (i, v) in field.iter().enumerate() {

        let fixed_blocks = v != &store::statics::Number::CURRENT && v != &store::statics::Number::EMPTY;

        if fixed_blocks {
            for w in tmp_block {
                if i == *w as usize {
                    is_filled = false;
                }
            }
        }
    }


    if !left_wall && !right_wall || !down_wall || !is_filled {
        false
    } else {
        true
    }
}
