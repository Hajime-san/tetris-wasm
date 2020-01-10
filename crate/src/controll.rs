#[path = "data.rs"]
mod data;
#[path = "func.rs"]
mod func;

#[derive(Debug)]
pub struct Field {
    field: data::Field,
}

impl Field {
    pub fn init() -> data::Field {
        func::init_vector(data::FIELD_LENGTH as usize)
    }
}

pub fn main() {
    let mut init_field: data::Field = Field::init();
}
