use math::round;
use std::f64::consts::PI;
// use rand::seq::SliceRandom;
// use rand::thread_rng;
mod data;

// for compare the first digit between field and a block
pub fn fix_digit(num: i32) -> i32 {
  if num < 0 {
    panic!("parameter {} is not a natural number", num);
  }

  if num < data::Number::DEGREES {
    return num;
  }

  let mut m_num: f64 = num as f64;

  let limited_digit: f64;

  loop {
    m_num = m_num / 10.0;

    if m_num >= 1.0 {
      limited_digit = m_num;
      break;
    }
  }

  // cut fisrt digit
  let fixed_num: f64 = round::floor(limited_digit, 0) * 10.0;

  let result: i32 = num % fixed_num as i32;

  return result;
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

  return rect;
}

pub fn rotate_matrix(rect: [i32; 2]) -> [i32; 2] {
  const RADIANS: f64 = (PI / 180.0) * data::Number::DEGREES as f64;
  let cos = RADIANS.cos() as i32;
  let sin = RADIANS.sin() as i32;
  let nx = (cos * (rect[0] - 0)) + (sin * (rect[1] - 0));
  let ny = (cos * (rect[1] - 0)) - (sin * (rect[0] - 0));
  return [nx, ny];
}

pub fn translate_rect_to_num(mat2: [i32; 2]) -> i32 {
  let point: i32;

  if mat2[0] == 0 && mat2[1] > 0 {
    point = -(mat2[1] * data::Number::ROW);
  } else if mat2[0] == 0 && mat2[1] < 0 {
    point = -(mat2[1] * data::Number::ROW);
  } else if mat2[0] == 0 && mat2[1] > 0 {
    point = mat2[1] * data::Number::ROW;
  } else if mat2[0] > 0 && mat2[1] == 0 {
    point = mat2[0];
  } else if mat2[0] < 0 && mat2[1] == 0 {
    point = mat2[0];
  } else if mat2[0] > 0 && mat2[1] > 0 {
    point = -(mat2[1] * data::Number::ROW) + mat2[0];
  } else if mat2[0] > 0 && mat2[1] < 0 {
    point = -(mat2[1] * data::Number::ROW) + mat2[0];
  } else if mat2[0] < 0 && mat2[1] > 0 {
    point = -(mat2[1] * data::Number::ROW) + mat2[0];
  } else if mat2[0] < 0 && mat2[1] < 0 {
    point = -(mat2[1] * data::Number::ROW) + mat2[0];
  } else if mat2[0] == 0 && mat2[1] == 0 {
    point = 0;

  //  undefined case
  } else {
    point = 0;
  }

  return point;
}

fn main() {
  let num = translate_number_to_rect(16, 15);
  let rect = rotate_matrix(num);
  let update = translate_rect_to_num(rect);

  println!("{:?} is num", num);
  println!("{:?} is rect", rect);
  println!("{} is rotated position", update);

  println!("{:?}", data::BLOCKS[0].number);

  // let choices = [1, 2, 4, 8, 16, 32];
  // let mut rng = thread_rng();
  // println!("{:?}", choices.choose(&mut rng));
}
