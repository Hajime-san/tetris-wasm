use arraytools::ArrayTools;

use crate::store;

#[derive(Debug)]
pub struct Block {
    pub positions: store::statics::BlockPosition,
    pub angle: i32,
    pub block_type: i32,
}

trait Update {
    fn transfer_to_fix(&mut self, mut field: Vec<i32>);
    fn clear_current(&mut self, mut field: Vec<i32>);
    fn move_current(&mut self, dir: &str) -> Result<store::statics::BlockPosition, ()>;
    fn transfer_current(
        &mut self,
        current_block: store::statics::BlockPosition,
        mut field: Vec<i32>,
    );
}

impl Default for Block {
    fn default() -> Self {
        Self {
            positions: [5, 6, 14, 15],
            angle: 0,
            block_type: 0,
        }
    }
}

impl Update for Block {
    fn clear_current(&mut self, mut field: Vec<i32>) {
        for v in &self.positions {
            field[*v as usize] = store::statics::Number::EMPTY;
        }
    }

    fn move_current(&mut self, dir: &str) -> Result<store::statics::BlockPosition, ()> {
        match dir {
            "left" => Ok(self
                .positions
                .map(|x| x + store::statics::Number::LEFT_MOVE)),
            "right" => Ok(self
                .positions
                .map(|x| x + store::statics::Number::RIGHT_MOVE)),
            "down" => Ok(self.positions.map(|x| x + store::statics::Number::ROW)),
            dir => Err(eprint!("wrong parameter '{}' is assinged!! ", dir)),
        }
    }

    fn transfer_current(
        &mut self,
        current_block: store::statics::BlockPosition,
        mut field: Vec<i32>,
    ) {
        self.positions = current_block;
        for v in &current_block {
            field[*v as usize] = store::statics::Number::CURRENT;
        }
    }

    fn transfer_to_fix(&mut self, mut field: Vec<i32>) {
        let iter_field = field.clone();
        for (i, v) in iter_field.iter().enumerate() {
            if *v == store::statics::Number::CURRENT {
                field[i as usize] = self.block_type;
            }
        }
    }
}
