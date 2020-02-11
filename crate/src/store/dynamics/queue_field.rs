use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::store;

#[derive(Debug)]
pub struct QueueField {
    pub numbers: Vec<i32>,
    pub line: Vec<i32>,
}

trait Update {
    fn create_queue(&mut self, count: i32);
}

impl Default for QueueField {
    fn default() -> Self {
        Self {
            numbers: default_queue_field_value(),
            line: default_queue_value(),
        }
    }
}

impl Update for QueueField {
    fn create_queue(&mut self, count: i32) {
        // delete first block
        if count >= 1 {
            self.line.drain(0..1);
        }
        // pick random between especially vector length
        if self.line.len() > 0 && self.line.len() < 4 {
            fn random() -> i32 {
                let mut rng = thread_rng();
                let r: i32 = rng.gen_range(0, &store::statics::BLOCKS.len()) as i32;
                r
            }

            loop {
                let serve = random();

                if !self.line.contains(&serve) {
                    self.line.push(serve);
                    break;
                }
            }
        }
    }
}

#[allow(unused_assignments)]
fn default_queue_field_value() -> Vec<i32> {
    let mut q = Vec::with_capacity(
        (store::statics::Number::QUEUE_ROW * store::statics::Number::QUEUE_ROW) as usize,
    );
    q = vec![
        store::statics::Number::EMPTY;
        (store::statics::Number::QUEUE_ROW * store::statics::Number::QUEUE_ROW) as usize
    ];

    q
}

pub fn default_queue_value() -> Vec<i32> {
    let mut queue: Vec<i32> = (0..store::statics::BLOCKS.len() as i32).collect();
    let mut rng = thread_rng();
    queue.shuffle(&mut rng);

    queue
}
