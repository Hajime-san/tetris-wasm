pub struct Number {}
impl Number {
    pub const EMPTY: i32 = -1;
    pub const CURRENT: i32 = 99;
    pub const ROW: i32 = 10;
    pub const COLUMN: i32 = 16;
    pub const A: i32 = 4;
    pub const DOWN_KEY: i32 = 40;
    pub const HARD_DOWN_KEY: i32 = 90;
    pub const UP_KEY: i32 = 38;
    pub const LEFT_KEY: i32 = 37;
    pub const RIGHT_KEY: i32 = 39;
    pub const ENTER_KEY: i32 = 13;
    pub const PAUSE_KEY: i32 = 80;
    pub const LEFT_MOVE: i32 = -1;
    pub const RIGHT_MOVE: i32 = 1;
    pub const DEGREES: i32 = 90;
    pub const QUEUE_ROW: i32 = 6;
    pub const QUEUE_COLUMN: i32 = 8;
}

#[derive(Debug, PartialEq)]
pub enum Angle {
    Initial = 0,
    Right = Number::DEGREES as isize,
    Down = (Number::DEGREES * 2) as isize,
    Left = (Number::DEGREES * 3) as isize,
}


#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Down
}

pub struct Text {}
impl Text {
    pub const EMPTY: &'static str = "empty";
    pub const CURRENT: &'static str = "current";
    pub const LEFT: &'static str = "left";
    pub const RIGHT: &'static str = "right";
    pub const DOWN: &'static str = "down";
}

#[derive(Debug)]
pub struct Prop {
    pub number: BlockPosition,
    pub color: &'static str,
    pub name: BlockName
}

pub type BlockPosition = [i32; 4];

#[derive(Debug, Clone, Copy)]
pub enum BlockName {
    O_mino,
    I_mino,
    J_mino,
    L_mino,
    T_mino,
    S_mino,
    Z_mino
}

pub const BLOCKS: [Prop; 7] = [
    Prop {
        number: O_NUMBER,
        color: O_COLOR,
        name: BlockName::O_mino
    },
    Prop {
        number: I_NUMBER,
        color: I_COLOR,
        name: BlockName::I_mino
    },
    Prop {
        number: J_NUMBER,
        color: J_COLOR,
        name: BlockName::J_mino
    },
    Prop {
        number: L_NUMBER,
        color: L_COLOR,
        name: BlockName::L_mino
    },
    Prop {
        number: T_NUMBER,
        color: T_COLOR,
        name: BlockName::T_mino
    },
    Prop {
        number: S_NUMBER,
        color: S_COLOR,
        name: BlockName::S_mino
    },
    Prop {
        number: Z_NUMBER,
        color: Z_COLOR,
        name: BlockName::Z_mino
    },
];

// O-block
pub const O_NUMBER: BlockPosition = [
    Number::A,
    Number::A + 1,
    Number::A + Number::ROW,
    Number::A + Number::ROW + 1,
];
pub const O_COLOR: &'static str = "rgba(240, 241, 77, 1)";

// I-block
pub const I_NUMBER: BlockPosition = [
    Number::A,
    Number::A + Number::ROW,
    Number::A + (Number::ROW * 2),
    Number::A + (Number::ROW * 3),
];
pub const I_COLOR: &'static str = "rgba(105, 241, 240, 1)";

// J-block
pub const J_NUMBER: BlockPosition = [
    Number::A,
    Number::A + 1,
    Number::A + Number::ROW,
    Number::A + (Number::ROW * 2),
];
pub const J_COLOR: &'static str = "rgba(27, 68, 241, 1)";

// L-block
pub const L_NUMBER: BlockPosition = [
    Number::A,
    Number::A + 1,
    Number::A + Number::ROW + 1,
    Number::A + (Number::ROW * 2) + 1,
];
pub const L_COLOR: &'static str = "rgba(240, 161, 63, 1)";

// T-block
pub const T_NUMBER: BlockPosition = [
    Number::A,
    Number::A + Number::ROW,
    Number::A + Number::ROW + 1,
    Number::A + (Number::ROW * 2),
];
pub const T_COLOR: &'static str = "rgba(163, 77, 240, 1)";

// S-block
pub const S_NUMBER: BlockPosition = [
    Number::A,
    Number::A + Number::ROW,
    Number::A + Number::ROW + 1,
    Number::A + (Number::ROW * 2) + 1,
];
pub const S_COLOR: &'static str = "rgba(114, 242, 63, 1)";

// Z-block
pub const Z_NUMBER: BlockPosition = [
    Number::A + 1,
    Number::A + Number::ROW,
    Number::A + Number::ROW + 1,
    Number::A + (Number::ROW * 2),
];
pub const Z_COLOR: &'static str = "rgba(237, 56, 51, 1)";
