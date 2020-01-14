use math::round;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::convert::TryInto;

use crate::store;

pub const FIELD_LENGTH: i32 = store::statics::Number::ROW * store::statics::Number::COLUMN;

#[derive(Debug)]
pub struct Config {
    pub field: Vec<i32>,
    pub queue_field: Vec<i32>,
    pub queue: Vec<i32>,
    pub angle: i32,
    pub current: store::statics::BlockPosition,
    pub count: i32,
    pub completed_row: i32,
    pub speed: i32,
    pub level: i32,
    pub score: i32,
    pub one_row_array: Vec<[i32; 2]>,
    pub complete_row_numbers: Vec<i32>,
    pub remain_row_array: Vec<[i32; 2]>,
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

impl Default for Config {
    fn default() -> Self {
        Self {
            field: default_field_value(),
            queue_field: default_queue_field_value(),
            queue: default_queue_value(),
            angle: 0,
            current: [0, 0, 0, 0],
            count: 1,
            completed_row: 0,
            speed: 1000,
            level: 0,
            score: 0,
            one_row_array: vec![[0; 2]],
            complete_row_numbers: vec![],
            remain_row_array: vec![[0; 2]],
            remain_row_numbers: vec![],
        }
    }
}

trait Update {
    fn create_queue(&self) -> Vec<i32>;
}

impl Update for Config {
    fn create_queue(&self) -> Vec<i32> {
        let mut queue = self.queue.clone();

        // delete first block
        if self.count >= 1 {
            queue.drain(0..1);
        }
        // pick random between especially vector length
        if queue.len() > 0 && queue.len() < 4 {
            fn random() -> i32 {
                let mut rng = thread_rng();
                let r: i32 = rng
                    .gen_range(0, &store::statics::BLOCKS.len())
                    .try_into()
                    .unwrap();
                r
            }

            while true {
                let serve = random();

                if !queue.contains(&serve) {
                    queue.push(serve);
                    break;
                }
            }
        }
        queue
    }
}

#[cfg(test)]
mod tests {
    use super::store;

    use super::Config;

    use super::Update;

    #[test]
    fn some_test() {
        let options: Config = Default::default();

        let queue = options.create_queue();

        println!("{:?}", queue);
    }
}

pub fn main() {}
