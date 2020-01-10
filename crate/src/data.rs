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

pub struct Text {}
impl Text {
    pub const EMPTY: &'static str = "empty";
    pub const CURRENT: &'static str = "current";
    pub const LEFT: &'static str = "left";
    pub const RIGHT: &'static str = "right";
    pub const DOWN: &'static str = "down";
}

pub const BLOCKS: [Prop; 7] = [
    Prop {
        number: O_NUMBER,
        color: O_COLOR,
    },
    Prop {
        number: I_NUMBER,
        color: I_COLOR,
    },
    Prop {
        number: J_NUMBER,
        color: J_COLOR,
    },
    Prop {
        number: L_NUMBER,
        color: L_COLOR,
    },
    Prop {
        number: T_NUMBER,
        color: T_COLOR,
    },
    Prop {
        number: S_NUMBER,
        color: S_COLOR,
    },
    Prop {
        number: Z_NUMBER,
        color: Z_COLOR,
    },
];

#[derive(Debug)]
pub struct Prop {
    pub number: BlockPosition,
    pub color: &'static str,
}

pub type BlockPosition = [i32; 4];

pub const FIELD_LENGTH: i32 = Number::ROW * Number::COLUMN;

pub type Field = Vec<i32>;

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
pub const I_COLOR: &'static str = "rgba(240, 241, 77, 1)";

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

fn main() {
    println!("{:?}", BLOCKS);
}
