use arraytools::ArrayTools;
use std::iter::IntoIterator;

use crate::func;
use crate::store;

#[derive(Debug)]
pub struct Block {
    pub positions: store::statics::BlockPosition,
    pub angle: store::statics::Angle,
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
    fn update_angle(&mut self);
    fn crate_rotate_block(&self, fix: bool) -> store::statics::BlockPosition;
}

impl Default for Block {
    fn default() -> Self {
        Self {
            positions: [5, 6, 14, 15],
            angle: store::statics::Angle::Right,
            block_type: 5,
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

    fn update_angle(&mut self) {
        match self.angle {
            store::statics::Angle::Initial => self.angle = store::statics::Angle::Right,
            store::statics::Angle::Right => self.angle = store::statics::Angle::Down,
            store::statics::Angle::Down => self.angle = store::statics::Angle::Left,
            store::statics::Angle::Left => self.angle = store::statics::Angle::Initial,
        }
    }

    fn crate_rotate_block(&self, fix: bool) -> store::statics::BlockPosition {
        // position of organization point
        let mut center = 0;
        // fix position after rotated
        let mut fix_position = 0;

        match self.block_type {
            0 => {
                if fix {
                    self.positions
                } else {
                    store::statics::BLOCKS[0].number
                };
            }
            1 => {
                center = 1;

                match self.angle {
                    store::statics::Angle::Initial => fix_position = 0,
                    store::statics::Angle::Left => fix_position = 0,
                    _ => fix_position = 1,
                }
            }
            2 => {
                fix_position = 0;

                match self.angle {
                    store::statics::Angle::Initial => center = 2,
                    store::statics::Angle::Right => center = 2,
                    _ => center = 1,
                }
            }
            3 => {
                fix_position = 0;

                match self.angle {
                    store::statics::Angle::Initial => center = 1,
                    store::statics::Angle::Right => center = 2,
                    store::statics::Angle::Down => center = 2,
                    store::statics::Angle::Left => center = 1,
                }
            }
            4 => {
                fix_position = 0;

                match self.angle {
                    store::statics::Angle::Initial => center = 0,
                    store::statics::Angle::Right => center = 2,
                    store::statics::Angle::Down => center = 3,
                    store::statics::Angle::Left => center = 1,
                }
            }
            5 => {
                fix_position = 0;

                match self.angle {
                    store::statics::Angle::Initial => {
                        center = 3;
                        fix_position =
                            -(store::statics::Number::ROW + store::statics::Number::RIGHT_MOVE);
                    }
                    store::statics::Angle::Right => center = 2,
                    store::statics::Angle::Down => center = 3,
                    store::statics::Angle::Left => {
                        center = 2;
                        fix_position =
                            store::statics::Number::ROW + store::statics::Number::LEFT_MOVE;
                    }
                }
            }
            6 => {
                center = 2;
                fix_position = 0;

                match self.angle {
                    store::statics::Angle::Initial => {
                        fix_position =
                            -(store::statics::Number::ROW + store::statics::Number::RIGHT_MOVE)
                    }
                    store::statics::Angle::Left => {
                        fix_position =
                            store::statics::Number::ROW + store::statics::Number::LEFT_MOVE
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        // for queue rendering option
        if !fix {
            fix_position = 0;
        }

        // create block from above information
        let mut rotate_blocks = [0; 4];

        for (i, v) in self.positions.iter().enumerate() {
            let num = func::translate_number_to_rect(*v, self.positions[center as usize]);
            let rect = func::rotate_matrix(num);
            let update = func::translate_rect_to_num(rect);
            let sum = update + self.positions[center as usize] + fix_position;
            rotate_blocks[i as usize] = sum;
        }
        rotate_blocks.sort();

        rotate_blocks
    }
}

// #[cfg(test)]
// mod tests {
//     use super::Block;
//     use super::Update;

//     #[test]
//     fn some_test() {
//         let def: Block = Default::default();

//         let rotate = def.crate_rotate_block(true);

//         println!("{:?}", rotate);
//     }
// }
