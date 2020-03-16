use crate::store;
use math::round;
use std::f64::consts::PI;
use store::dynamics::render::element::Canvas;

// for compare the first digit between field and a block
pub fn fix_digit(num: i32) -> i32 {
    let mut digits = Vec::new();
    let mut num = num;
    while num > 9 {
        digits.push(num % 10);
        num = num / 10;
    }
    digits.push(num);
    digits.reverse();

    digits[digits.len() - 1]
}

pub fn translate_number_to_rect(num: i32, center: i32) -> [i32; 2] {
    let mut rect = [0, 0];

    if num <= center && (center - num) <= 2 {
        rect = [fix_digit(num) - fix_digit(center), 0];
    } else if num >= center && (num - center) <= 2 {
        rect = [fix_digit(num) - fix_digit(center), 0];
    } else if num <= center && (center - num) >= 3 && (center - num) <= 13 {
        rect = [fix_digit(num) - fix_digit(center), 1];
    } else if num >= center && (num - center) >= 3 && (num - center) <= 13 {
        rect = [fix_digit(num) - fix_digit(center), -1];
    } else if num <= center && (center - num) >= 3 && (center - num) >= 13 {
        rect = [fix_digit(num) - fix_digit(center), 2];
    } else if num >= center && (num - center) >= 3 && (num - center) >= 13 {
        rect = [fix_digit(num) - fix_digit(center), -2];
    }

    rect
}

pub fn rotate_matrix(rect: [i32; 2]) -> [i32; 2] {
    const RADIANS: f64 = (PI / 180.0) * store::statics::Number::DEGREES as f64;
    let cos = RADIANS.cos() as i32;
    let sin = RADIANS.sin() as i32;
    let nx = (cos * (rect[0] - 0)) + (sin * (rect[1] - 0));
    let ny = (cos * (rect[1] - 0)) - (sin * (rect[0] - 0));

    [nx, ny]
}

pub fn translate_rect_to_num(mat2: [i32; 2]) -> i32 {
    let point: i32;

    if mat2[0] == 0 && mat2[1] > 0 {
        point = -(mat2[1] * store::statics::Number::ROW);
    } else if mat2[0] == 0 && mat2[1] < 0 {
        point = -(mat2[1] * store::statics::Number::ROW);
    } else if mat2[0] == 0 && mat2[1] > 0 {
        point = mat2[1] * store::statics::Number::ROW;
    } else if mat2[0] > 0 && mat2[1] == 0 {
        point = mat2[0];
    } else if mat2[0] < 0 && mat2[1] == 0 {
        point = mat2[0];
    } else if mat2[0] > 0 && mat2[1] > 0 {
        point = -(mat2[1] * store::statics::Number::ROW) + mat2[0];
    } else if mat2[0] > 0 && mat2[1] < 0 {
        point = -(mat2[1] * store::statics::Number::ROW) + mat2[0];
    } else if mat2[0] < 0 && mat2[1] > 0 {
        point = -(mat2[1] * store::statics::Number::ROW) + mat2[0];
    } else if mat2[0] < 0 && mat2[1] < 0 {
        point = -(mat2[1] * store::statics::Number::ROW) + mat2[0];
    } else if mat2[0] == 0 && mat2[1] == 0 {
        point = 0;

    //  undefined case
    } else {
        point = 0;
    }

    point
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use crate::store::dynamics::render::element::Get;
//     use crate::store::dynamics::render::element::Update;

//     #[test]
//     fn test() {
//         let num = translate_number_to_rect(5, 15);
//         let rect = rotate_matrix(num);
//         let update = translate_rect_to_num(rect);

//         println!("{:?} is num", num);
//         println!("{:?} is rect", rect);
//         println!("{} is rotated position", update);

//         let mut canvas: Canvas = Default::default();
//         canvas.set_width(400);
//         canvas.set_height(600);

//         println!("{:?}", canvas.get_width());
//     }
// }
