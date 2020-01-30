use arraytools::ArrayTools;
use math::round;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::convert::TryInto;

use crate::store;

pub const FIELD_LENGTH: i32 = store::statics::Number::ROW * store::statics::Number::COLUMN;

#[derive(Debug)]
pub struct Field {
    pub field: Vec<i32>,
    pub queue_field: Vec<i32>,
    pub queue: Vec<i32>,
    pub angle: i32,
    pub block_number: i32,
    pub current: store::statics::BlockPosition,
    pub count: i32,
    pub completed_row: i32,
    pub speed: i32,
    pub level: i32,
    pub score: i32,
    pub single_row: Vec<[i32; store::statics::Number::ROW as usize]>,
    pub complete_row_numbers: Vec<i32>,
    pub remain_row: Vec<[i32; store::statics::Number::ROW as usize]>,
    pub remain_row_numbers: Vec<i32>,
}

fn default_field_value() -> Vec<i32> {
    let mut f = Vec::with_capacity(FIELD_LENGTH.try_into().unwrap());
    f = vec![store::statics::Number::EMPTY; FIELD_LENGTH.try_into().unwrap()];

    f
}

fn default_queue_field_value() -> Vec<i32> {
    let mut q = Vec::with_capacity(
        (store::statics::Number::QUEUE_ROW * store::statics::Number::QUEUE_ROW)
            .try_into()
            .unwrap(),
    );
    q = vec![
        store::statics::Number::EMPTY;
        (store::statics::Number::QUEUE_ROW * store::statics::Number::QUEUE_ROW)
            .try_into()
            .unwrap()
    ];

    q
}

pub fn default_queue_value() -> Vec<i32> {
    let mut queue: Vec<i32> = (0..store::statics::BLOCKS.len() as i32).collect();
    let mut rng = thread_rng();
    queue.shuffle(&mut rng);

    queue
}

impl Default for Field {
    fn default() -> Self {
        Self {
            field: default_field_value(),
            queue_field: default_queue_field_value(),
            queue: default_queue_value(),
            angle: 0,
            block_number: 0,
            current: [5, 6, 14, 15],
            count: 1,
            completed_row: 0,
            speed: 1000,
            level: 0,
            score: 0,
            single_row: vec![],
            complete_row_numbers: vec![],
            remain_row: vec![],
            remain_row_numbers: vec![],
        }
    }
}

trait Update {
    fn create_queue(&mut self);
    fn delete_row(&mut self);
    fn drop_row(&mut self);
    fn transfer_to_fix(&mut self);
    fn clear_current(&mut self);
    fn move_current(&mut self, dir: &str) -> Result<store::statics::BlockPosition, ()>;
    fn transfer_current(&mut self, current_block: store::statics::BlockPosition);
}

impl Update for Field {
    fn create_queue(&mut self) {
        // delete first block
        if self.count >= 1 {
            self.queue.drain(0..1);
        }
        // pick random between especially vector length
        if self.queue.len() > 0 && self.queue.len() < 4 {
            fn random() -> i32 {
                let mut rng = thread_rng();
                let r: i32 = rng
                    .gen_range(0, &store::statics::BLOCKS.len())
                    .try_into()
                    .unwrap();
                r
            }

            loop {
                let serve = random();

                if !self.queue.contains(&serve) {
                    self.queue.push(serve);
                    break;
                }
            }
        }
    }

    fn delete_row(&mut self) {
        // create arrays for deleting rows
        let mut delete_row_array: Vec<i32> = vec![]; // should delete areas

        // create arrays for moving blocks after deleted
        for (i, v) in self.single_row.iter().enumerate() {
            let check_empty = v.iter().any(|&x| x == store::statics::Number::EMPTY);
            let check_current = v.iter().any(|&x| x == store::statics::Number::CURRENT);
            let check_fixed = v.iter().any(|&x| {
                x != store::statics::Number::EMPTY && x != store::statics::Number::CURRENT
            });
            let check_rows = v.iter().all(|&x| x != store::statics::Number::EMPTY);

            if (check_empty && check_fixed) || check_current && !check_rows {
                self.remain_row.push(*v);
                self.remain_row_numbers.push(i.try_into().unwrap());
            }

            if !check_rows {
                return;
            }

            self.complete_row_numbers.push(i.try_into().unwrap());

            for j in 0..store::statics::Number::ROW {
                delete_row_array.push((i as i32 * store::statics::Number::ROW) + j as i32);
            }
        }

        // delete complete rows
        for v in delete_row_array {
            self.field[v as usize] = store::statics::Number::EMPTY;
        }
    }

    fn drop_row(&mut self) {
        // first, drop remainRow

        if self.remain_row_numbers.len() > 0 {
            let mut reverse_num: Vec<i32> = self.remain_row_numbers.iter().rev().cloned().collect();
            let reverse_row: Vec<[i32; 10]> = self.remain_row.iter().rev().cloned().collect();

            for _ in 0..store::statics::Number::COLUMN.try_into().unwrap() {
                // loop
                for (i, v) in reverse_num.iter().enumerate() {
                    let start = (v * store::statics::Number::ROW) + store::statics::Number::ROW;

                    if start >= self.field.len().try_into().unwrap() {
                        return;
                    }

                    let check: Vec<i32> = self
                        .field
                        .iter()
                        .filter(|&k| k >= &start && k <= &(start + store::statics::Number::ROW - 1))
                        .cloned()
                        .collect();

                    if check
                        .iter()
                        .all(|&x| self.field[x as usize] == store::statics::Number::EMPTY)
                    {
                        for j in 0..store::statics::Number::ROW {
                            let value = reverse_row[i as usize][j as usize];

                            self.field[(start + j) as usize] = value;
                            self.field[((start - store::statics::Number::ROW) + j) as usize] =
                                store::statics::Number::EMPTY;
                        }
                    }
                }

                // increment
                for j in 0..reverse_num.len().try_into().unwrap() {
                    if reverse_num[j as usize] >= store::statics::Number::COLUMN {
                        return;
                    }

                    reverse_num[j as usize] += 1;
                }
            }
        }
    }

    fn clear_current(&mut self) {
        for v in &self.current {
            self.field[*v as usize] = store::statics::Number::EMPTY;
        }
    }

    fn move_current(&mut self, dir: &str) -> Result<store::statics::BlockPosition, ()> {
        match dir {
            "left" => Ok(self.current.map(|x| x + store::statics::Number::LEFT_MOVE)),
            "right" => Ok(self.current.map(|x| x + store::statics::Number::RIGHT_MOVE)),
            "down" => Ok(self.current.map(|x| x + store::statics::Number::ROW)),
            dir => Err(eprint!("wrong parameter '{}' is assinged!! ", dir)),
        }
    }

    fn transfer_current(&mut self, current_block: store::statics::BlockPosition) {
        self.current = current_block;
        for v in &current_block {
            self.field[*v as usize] = store::statics::Number::CURRENT;
        }
    }

    fn transfer_to_fix(&mut self) {
        let field = self.field.clone();
        for (i, v) in field.iter().enumerate() {
            if *v == store::statics::Number::CURRENT {
                self.field[i as usize] = self.block_number;
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::store;

//     use super::Field;

//     use super::Update;

//     #[test]
//     fn some_test() {
//         let mut options: Field = Default::default();

//         let queue = options.create_queue();

//         options.single_row = vec![[99, -1, 1, 1, 1, 1, 1, 1, 1, 1]];

//         options.delete_row();

//         options.current = options.move_current("down").unwrap();

//         options.transfer_current(options.current);

//         options.transfer_to_fix();

//         println!("{:?}", options.field);
//     }
// }

pub fn main() {}
