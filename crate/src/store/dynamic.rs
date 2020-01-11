use crate::func;
use crate::store;

#[derive(Debug, Default)]
pub struct Field {
    field: store::r#static::Field,
}

impl Field {
    pub fn init() -> store::r#static::Field {
        func::init_vector(store::r#static::FIELD_LENGTH as usize)
    }

    pub fn field(&self) -> &store::r#static::Field {
        &self.field
    }
}
