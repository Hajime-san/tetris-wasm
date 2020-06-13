
/// for compare the first digit between field and a block
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
