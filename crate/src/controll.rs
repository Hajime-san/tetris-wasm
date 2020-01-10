#[path = "data.rs"]
mod data;
#[path = "func.rs"]
mod func;

#[derive(Debug, Default)]
pub struct Field {
    field: data::Field,
}

impl Field {
    pub fn init() -> data::Field {
        func::init_vector(data::FIELD_LENGTH as usize)
    }

    pub fn field(&self) -> &data::Field {
        &self.field
    }
}

fn main() {}
