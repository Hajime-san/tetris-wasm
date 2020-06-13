use crate::store;

#[derive(Debug)]
pub enum Complete {
    Failure,
    Success,
}

impl Complete {
    /// check completion at least one row
    pub fn check_complete(&self, single_rows: &Vec<Vec<i32>>) -> Complete {
        let mut flag = Self::Failure;

        // check row
        let check_rows = single_rows
                        .iter()
                        .any(|v| v
                            .iter()
                            .all(|&w| w != store::statics::Number::EMPTY));

        if check_rows {
            flag = Self::Success;
        }

        flag
    }
}
