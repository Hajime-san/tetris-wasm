use crate::func;
use crate::store;

use store::dynamics::field::Field as GameFieldContext;
use store::dynamics::block::Block as BlockContext;

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
            flag = true;
        }
    }

    flag
}

pub fn down(field_collection: &GameFieldContext, current_block: &store::statics::BlockPosition) -> bool {
    let mut flag = true;

    let field = field_collection.get_list();

    // check fixed block
    for (i, v) in field.iter().enumerate() {

        let index = i as i32 - store::statics::Number::ROW;
        if index < 0 {
            continue;
        }

        let skip_current = v != &store::statics::Number::CURRENT && v != &store::statics::Number::EMPTY;


            if &field[index as usize] == &store::statics::Number::CURRENT && skip_current {
                flag = false;
            }
    }

    // check last row
    for v in current_block {
        let is_last_row = field.iter().any(|&x| v >= &(field.len() as i32 - store::statics::Number::ROW));
        if is_last_row {
            flag = false;
        }
    }

    flag
}

pub fn rotate(field: &Vec<i32>, tmp_block: &store::statics::BlockPosition) -> bool {
    let mut left_wall = true;
    let mut right_wall = true;
    let mut down_wall = true;

    // wall check down/left/right
    for v in tmp_block {
        let is_last_row = field
            .iter()
            .any(|&x| x >= field.len() as i32 - store::statics::Number::ROW);

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
        if *v != store::statics::Number::CURRENT || *v != store::statics::Number::EMPTY {
            break;
        }

        for w in tmp_block {
            if i == *w as usize {
                is_filled = false;
            }
        }
    }

    if !left_wall && !right_wall || !down_wall || !is_filled {
        false
    } else {
        true
    }
}
