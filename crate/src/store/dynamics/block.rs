use arraytools::ArrayTools;

use crate::util;
use crate::store;
use std::f64::consts::PI;

#[derive(Debug)]
/// store data for playing block
pub struct BlockContext {
    positions: store::statics::BlockPosition,
    angle: store::statics::Angle,
    next_angle: store::statics::Angle,
    block_name: store::statics::BlockName,
}


impl BlockContext {

    /*
        constructor
    */

    pub fn new( positions: store::statics::BlockPosition,
                block_name: store::statics::BlockName) -> Self {
        Self {
            positions: positions,
            angle: store::statics::Angle::Initial,
            next_angle: store::statics::Angle::Initial,
            block_name: block_name,
        }
    }

    //
    // get field value methods
    //

    pub fn get_current_block_positions(&self) -> store::statics::BlockPosition {
        self.positions
    }

    pub fn get_current_block_name(&self) -> store::statics::BlockName {
        self.block_name
    }

    /// return after moved block positions
    pub fn get_moved_current_block_positions(&mut self, dir: store::statics::Direction) -> store::statics::BlockPosition {
        match dir {
            store::statics::Direction::Left => self
                .positions
                .map(|x| x + store::statics::Number::LEFT_MOVE),
            store::statics::Direction::Right => self
                .positions
                .map(|x| x + store::statics::Number::RIGHT_MOVE),
            store::statics::Direction::Down => self.positions.map(|x| x + store::statics::Number::ROW),
        }
    }

    /// create rotate block by next angle or current angle
    pub fn crate_rotate_block(&mut self, option: &str, fix: bool) -> store::statics::BlockPosition {

        // update angle
        if option == "fixed" {
            match self.angle {
                store::statics::Angle::Initial => self.angle = store::statics::Angle::Right,
                store::statics::Angle::Right => self.angle = store::statics::Angle::Down,
                store::statics::Angle::Down => self.angle = store::statics::Angle::Left,
                store::statics::Angle::Left => self.angle = store::statics::Angle::Initial,
            }
        } else {
            // emurate next angle
            match self.angle {
                store::statics::Angle::Initial => self.next_angle = store::statics::Angle::Right,
                store::statics::Angle::Right => self.next_angle = store::statics::Angle::Down,
                store::statics::Angle::Down => self.next_angle = store::statics::Angle::Left,
                store::statics::Angle::Left => self.next_angle = store::statics::Angle::Initial,
            }
        }

        // position of organization point
        let mut center = 0;
        // fix position after rotated
        let mut fix_position = 0;

        match &self.block_name {
            store::statics::BlockName::O_mino => {
                if fix {
                    // update current positions
                    fix_position = 1;

                    self.positions
                } else {
                    store::statics::BLOCKS[store::statics::BlockName::O_mino as usize].number
                };
            }
            store::statics::BlockName::I_mino => {
                center = 1;

                if option == "fixed" {
                    match self.angle {
                        store::statics::Angle::Initial => fix_position = 0,
                        store::statics::Angle::Left => fix_position = 0,
                        _ => fix_position = 1,
                    }
                } else {
                    match self.next_angle {
                        store::statics::Angle::Initial => fix_position = 0,
                        store::statics::Angle::Left => fix_position = 0,
                        _ => fix_position = 1,
                    }
                }
            }
            store::statics::BlockName::J_mino => {
                fix_position = 0;

                if option == "fixed" {
                    match self.angle {
                        store::statics::Angle::Initial => center = 2,
                        store::statics::Angle::Right => center = 2,
                        _ => center = 1,
                    }
                } else {
                    match self.next_angle {
                        store::statics::Angle::Initial => center = 2,
                        store::statics::Angle::Right => center = 2,
                        _ => center = 1,
                    }
                }
            }
            store::statics::BlockName::L_mino => {
                fix_position = 0;

                if option == "fixed" {
                    match self.angle {
                        store::statics::Angle::Initial => center = 1,
                        store::statics::Angle::Right => center = 2,
                        store::statics::Angle::Down => center = 2,
                        store::statics::Angle::Left => center = 1,
                    }
                } else {
                    match self.next_angle {
                        store::statics::Angle::Initial => center = 1,
                        store::statics::Angle::Right => center = 2,
                        store::statics::Angle::Down => center = 2,
                        store::statics::Angle::Left => center = 1,
                    }
                }
            }
            store::statics::BlockName::T_mino => {
                fix_position = 0;

                if option == "fixed" {
                    match self.angle {
                        store::statics::Angle::Initial => center = 0,
                        store::statics::Angle::Right => center = 2,
                        store::statics::Angle::Down => center = 3,
                        store::statics::Angle::Left => center = 1,
                    }
                } else {
                    match self.next_angle {
                        store::statics::Angle::Initial => center = 0,
                        store::statics::Angle::Right => center = 2,
                        store::statics::Angle::Down => center = 3,
                        store::statics::Angle::Left => center = 1,
                    }
                }
            }
            store::statics::BlockName::S_mino => {
                fix_position = 0;

                if option == "fixed" {
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
                } else {
                    match self.next_angle {
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
            }
            store::statics::BlockName::Z_mino => {
                center = 2;
                fix_position = 0;

                if option == "fixed" {
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
                } else {
                    match self.next_angle {
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
        }

        // for queue rendering option
        if !fix {
            fix_position = 0;
        }

        // create block from above information
        let mut rotate_blocks = [0; 4];

        for (i, v) in self.positions.iter().enumerate() {
            let num = translate_number_to_rect(*v, self.positions[center as usize]);
            let rect = rotate_matrix(num);
            let translated_num = translate_rect_to_num(rect);
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
}


/// translate block position value to 2d rect
fn translate_number_to_rect(num: i32, center: i32) -> [i32; 2] {
    let mut rect = [0, 0];

    if num <= center && (center - num) <= 2 {
        rect = [util::fix_digit(num) - util::fix_digit(center), 0];
    } else if num >= center && (num - center) <= 2 {
        rect = [util::fix_digit(num) - util::fix_digit(center), 0];
    } else if num <= center && (center - num) >= 3 && (center - num) <= 13 {
        rect = [util::fix_digit(num) - util::fix_digit(center), 1];
    } else if num >= center && (num - center) >= 3 && (num - center) <= 13 {
        rect = [util::fix_digit(num) - util::fix_digit(center), -1];
    } else if num <= center && (center - num) >= 3 && (center - num) >= 13 {
        rect = [util::fix_digit(num) - util::fix_digit(center), 2];
    } else if num >= center && (num - center) >= 3 && (num - center) >= 13 {
        rect = [util::fix_digit(num) - util::fix_digit(center), -2];
    }

    rect
}

/// rotate 2d rect by angle
fn rotate_matrix(rect: [i32; 2]) -> [i32; 2] {
    const RADIANS: f64 = (PI / 180.0) * store::statics::Number::DEGREES as f64;
    let cos = RADIANS.cos() as i32;
    let sin = RADIANS.sin() as i32;
    let nx = (cos * (rect[0] - 0)) + (sin * (rect[1] - 0));
    let ny = (cos * (rect[1] - 0)) - (sin * (rect[0] - 0));

    [nx, ny]
}

// restore new block positions from 2d rect
fn translate_rect_to_num(mat2: [i32; 2]) -> i32 {
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
