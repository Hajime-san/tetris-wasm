use crate::store;

pub const FIELD_LENGTH: i32 = store::statics::Number::ROW * store::statics::Number::COLUMN;

#[derive(Debug)]
pub struct Field {
    pub numbers: Vec<i32>,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            numbers: default_field_value(),
        }
    }
}

fn default_field_value() -> Vec<i32> {
    let mut f = Vec::with_capacity(FIELD_LENGTH as usize);
    f = vec![store::statics::Number::EMPTY; FIELD_LENGTH as usize];

    f
}
