use crate::store;

pub const FIELD_LENGTH: i32 = store::statics::Number::ROW * store::statics::Number::COLUMN;

#[derive(Debug, Clone)]
pub struct Field(Vec<i32>);

impl Default for Field {
    fn default() -> Self {
        Self(default_field_value())
    }
}

pub trait Update {
    fn transfer_to_fix(&mut self, block_type: &i32);
    fn clear_current(&mut self, potisions: &store::statics::BlockPosition);
    fn transfer_current(&mut self, potisions: &store::statics::BlockPosition);
}

pub trait Get {
    fn get_numbers(&self) -> &Vec<i32>;
}

impl Update for Field {
    fn transfer_to_fix(&mut self, block_type: &i32) {
        let iter_field = self.clone();

        for (i, v) in iter_field.0.iter().enumerate() {
            if *v == store::statics::Number::CURRENT {
                self.0[i as usize] = *block_type;
            }
        }
    }

    fn clear_current(&mut self, potisions: &store::statics::BlockPosition) {
        for v in potisions {
            self.0[*v as usize] = store::statics::Number::EMPTY;
        }
    }

    fn transfer_current(&mut self, potisions: &store::statics::BlockPosition) {
        for v in potisions {
            self.0[*v as usize] = store::statics::Number::CURRENT;
        }
    }
}

impl Get for Field {
    fn get_numbers(&self) -> &Vec<i32> {
        &self.0
    }
}

#[allow(unused_assignments)]
fn default_field_value() -> Vec<i32> {
    let mut f = Vec::with_capacity(FIELD_LENGTH as usize);
    f = vec![store::statics::Number::EMPTY; FIELD_LENGTH as usize];

    f
}
