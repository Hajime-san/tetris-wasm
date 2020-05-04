use arraytools::ArrayTools;

use crate::func;
use crate::store;

#[derive(Debug)]
pub struct Block {
    positions: store::statics::BlockPosition,
    _positions: store::statics::BlockPosition,
    angle: store::statics::Angle,
    _angle: store::statics::Angle,
    block_type: store::statics::BlockName,
}


impl Default for Block {
    fn default() -> Self {
        Self {
            positions: [5, 6, 14, 15],
            _positions: [5, 15, 16, 26],
            angle: store::statics::Angle::Right,
            _angle: store::statics::Angle::Down,
            block_type: store::statics::BlockName::S_mino,
        }
    }
}

impl Block {

    //
    // get field value methods
    //

    pub fn get_current_block_positions(&self) -> store::statics::BlockPosition {
        self.positions
    }

    pub fn get_current_block_type(&self) -> store::statics::BlockName {
        self.block_type
    }

    pub fn get_moved_current_block_positions(&mut self, dir: &str) -> Result<store::statics::BlockPosition, ()> {
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

    pub fn crate_rotate_block(&mut self, fix: bool) -> store::statics::BlockPosition {
        // update angle
        match self.angle {
            store::statics::Angle::Initial => self.angle = store::statics::Angle::Right,
            store::statics::Angle::Right => self.angle = store::statics::Angle::Down,
            store::statics::Angle::Down => self.angle = store::statics::Angle::Left,
            store::statics::Angle::Left => self.angle = store::statics::Angle::Initial,
        }

        // position of organization point
        let mut center = 0;
        // fix position after rotated
        let mut fix_position = 0;

        match &self.block_type {
            store::statics::BlockName::O_mino => {
                if fix {
                    self.positions
                } else {
                    store::statics::BLOCKS[store::statics::BlockName::O_mino as usize].number
                };
            }
            store::statics::BlockName::I_mino => {
                center = 1;

                match self.angle {
                    store::statics::Angle::Initial => fix_position = 0,
                    store::statics::Angle::Left => fix_position = 0,
                    _ => fix_position = 1,
                }
            }
            store::statics::BlockName::J_mino => {
                fix_position = 0;

                match self.angle {
                    store::statics::Angle::Initial => center = 2,
                    store::statics::Angle::Right => center = 2,
                    _ => center = 1,
                }
            }
            store::statics::BlockName::L_mino => {
                fix_position = 0;

                match self.angle {
                    store::statics::Angle::Initial => center = 1,
                    store::statics::Angle::Right => center = 2,
                    store::statics::Angle::Down => center = 2,
                    store::statics::Angle::Left => center = 1,
                }
            }
            store::statics::BlockName::T_mino => {
                fix_position = 0;

                match self.angle {
                    store::statics::Angle::Initial => center = 0,
                    store::statics::Angle::Right => center = 2,
                    store::statics::Angle::Down => center = 3,
                    store::statics::Angle::Left => center = 1,
                }
            }
            store::statics::BlockName::S_mino => {
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
            store::statics::BlockName::Z_mino => {
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

                    // return fix_position = 0, that declared upper scope
                    _ => (),
                }
            }
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
            let translated_num = func::translate_rect_to_num(rect);
            let fixed_num = translated_num + self.positions[center as usize] + fix_position;
            rotate_blocks[i as usize] = fixed_num;
        }
        rotate_blocks.sort();

        rotate_blocks
    }


    //
    // update field value methods
    //

    pub fn update_current_positions(&mut self, moved_current_positions: &store::statics::BlockPosition) {
        self.positions = *moved_current_positions;
    }

    // when failure to rotate
    pub fn reverse_angle(&mut self) {
        match self.angle {
            store::statics::Angle::Initial => self.angle = store::statics::Angle::Left,
            store::statics::Angle::Right => self.angle = store::statics::Angle::Initial,
            store::statics::Angle::Down => self.angle = store::statics::Angle::Right,
            store::statics::Angle::Left => self.angle = store::statics::Angle::Down,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::Block;
//     use super::Update;
//     use super::Get;

//     #[test]
//     fn some_test() {
//         let def: Block = Default::default();

//         let rotate = def.get_current_block_number();

//         println!("{:?}", rotate);
//     }
// }
