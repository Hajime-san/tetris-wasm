use crate::store;

enum Complete {
    Failure,
    Success,
}


impl Complete {
    pub fn check_complete(&self, single_rows: &Vec<Vec<i32>>) -> Complete {
        let mut flag = Self::Failure;
        // check row
        for v in single_rows {
            let check_rows = v.iter().all(|&x| x != store::statics::Number::EMPTY);

            if check_rows {
                flag = Self::Success
            } else {
                flag = Self::Failure
            }
        }

        flag
    }
}
