use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Math;

use crate::store;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug)]
pub struct QueueField {
    list: Vec<i32>,
    line: Vec<i32>,
}

impl Default for QueueField {
    fn default() -> Self {
        Self {
            list: default_queue_field_value(),
            line: create_initial_queue(),
        }
    }
}

impl QueueField {
    pub fn update_queue(&mut self, count: i32) {
        // delete first block
        if count >= 1 {
            self.line.drain(0..1);
        }
        // pick random between especially vector length
        if self.line.len() > 0 && self.line.len() < 4 {
            fn random() -> i32 {
                let r = Math::floor(Math::random() * store::statics::BLOCKS.len() as f64) as i32;
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

    pub fn get_block_name(&self) -> store::statics::BlockName {
        match self.line[0] {
            _ => store::statics::BLOCKS[self.line[0] as usize].name
        }
    }

    pub fn get_line(&self) -> &Vec<i32> {
        &self.line
    }

    pub fn get_next_block(&self) -> i32 {
        self.line[1]
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


fn create_initial_queue() -> Vec<i32> {

    let mut queue = vec![];

    fn random() -> i32 {
        let r = Math::floor(Math::random() * store::statics::BLOCKS.len() as f64) as i32;
        r
    }

    loop {

        if queue.len() == store::statics::BLOCKS.len() {
            break;
        }

        let serve = random();

        if !queue.contains(&serve) {
            queue.push(serve);
        }
    }

    queue
}
