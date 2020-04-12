use crate::store;

pub const FIELD_LENGTH: i32 = store::statics::Number::ROW * store::statics::Number::COLUMN;

#[derive(Debug, Clone)]
pub struct Field {
    list: Vec<i32>,
    single_rows: Vec<Vec<i32>>,
    complete_row_numbers: Vec<i32>,
    remain_rows: Vec<Vec<i32>>,
    remain_row_numbers: Vec<i32>,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            list: default_field_value(),
            single_rows: vec![],
            complete_row_numbers: vec![],
            remain_rows: vec![],
            remain_row_numbers: vec![],
        }
    }
}

pub trait Update {
    fn transfer_to_fixed_number(&mut self, block_type: &i32);
    fn clear_current_block(&mut self, potisions: &store::statics::BlockPosition);
    fn transfer_current_block(&mut self, potisions: &store::statics::BlockPosition);
    fn create_single_rows(&mut self);
    fn delete_row(&mut self);
    fn drop_row(&mut self);
}

pub trait Get {
    fn get_list(&self) -> &Vec<i32>;
    fn get_complete_row_numbers(&self) -> &Vec<i32>;
}

impl Update for Field {
    fn transfer_to_fixed_number(&mut self, block_type: &i32) {
        let iter_field = self.list.clone();

        for (i, v) in iter_field.iter().enumerate() {
            if *v == store::statics::Number::CURRENT {
                self.list[i as usize] = *block_type;
            }
        }
    }

    fn clear_current_block(&mut self, potisions: &store::statics::BlockPosition) {
        for v in potisions {
            self.list[*v as usize] = store::statics::Number::EMPTY;
        }
    }

    fn transfer_current_block(&mut self, potisions: &store::statics::BlockPosition) {
        for v in potisions {
            self.list[*v as usize] = store::statics::Number::CURRENT;
        }
    }

    fn create_single_rows(&mut self) {
        // creat vector on each rows
        let mut start = 0;
        let mut end = store::statics::Number::ROW;

        for _ in 0..store::statics::Number::COLUMN {
            let row: Vec<i32> = (self.list[start as usize]..self.list[end as usize]).collect();
            start += store::statics::Number::ROW;
            end += store::statics::Number::ROW;
            self.single_rows.push(row);
        }
    }

    fn delete_row(&mut self) {
        // create arrays for deleting rows
        let mut delete_row_array: Vec<i32> = vec![]; // should delete areas

        // create arrays for moving blocks after deleted
        for (i, v) in self.single_rows.iter().enumerate() {
            let check_empty = v.iter().any(|&x| x == store::statics::Number::EMPTY);
            let check_current = v.iter().any(|&x| x == store::statics::Number::CURRENT);
            let check_fixed = v.iter().any(|&x| {
                x != store::statics::Number::EMPTY && x != store::statics::Number::CURRENT
            });
            let check_rows = v.iter().all(|&x| x != store::statics::Number::EMPTY);

            if (check_empty && check_fixed) || check_current && !check_rows {
                self.remain_rows.push(v.to_vec());
                self.remain_row_numbers.push(i as i32);
            }

            if !check_rows {
                return;
            }

            self.complete_row_numbers.push(i as i32);

            for j in 0..store::statics::Number::ROW {
                delete_row_array.push((i as i32 * store::statics::Number::ROW) + j as i32);
            }
        }

        // delete complete rows
        for v in delete_row_array {
            self.list[v as usize] = store::statics::Number::EMPTY;
        }
    }

    fn drop_row(&mut self) {
        // first, drop remainRow

        if self.remain_row_numbers.len() > 0 {
            let mut reverse_num: Vec<i32> = self.remain_row_numbers.iter().rev().cloned().collect();
            let reverse_rows: Vec<Vec<i32>> = self.remain_rows.iter().rev().cloned().collect();

            for _ in 0..store::statics::Number::COLUMN as usize {
                // loop
                for (i, v) in reverse_num.iter().enumerate() {
                    let start = (v * store::statics::Number::ROW) + store::statics::Number::ROW;

                    if start >= self.list.len() as i32 {
                        return;
                    }

                    let check: Vec<i32> = self.list
                        .iter()
                        .filter(|&k| k >= &start && k <= &(start + store::statics::Number::ROW - 1))
                        .cloned()
                        .collect();

                    if check
                        .iter()
                        .all(|&x| self.list[x as usize] == store::statics::Number::EMPTY)
                    {
                        for j in 0..store::statics::Number::ROW {
                            let value = reverse_rows[i as usize][j as usize];

                            self.list[(start + j) as usize] = value;
                            self.list[((start - store::statics::Number::ROW) + j) as usize] =
                                store::statics::Number::EMPTY;
                        }
                    }
                }

                // increment
                for j in 0..reverse_num.len() as usize {
                    if reverse_num[j as usize] >= store::statics::Number::COLUMN {
                        return;
                    }

                    reverse_num[j as usize] += 1;
                }
            }
        }
    }
}

impl Get for Field {
    fn get_list(&self) -> &Vec<i32> {
        &self.list
    }
    fn get_complete_row_numbers(&self) -> &Vec<i32> {
        &self.complete_row_numbers
    }
}

#[allow(unused_assignments)]
fn default_field_value() -> Vec<i32> {
    let mut f = Vec::with_capacity(FIELD_LENGTH as usize);
    f = vec![store::statics::Number::EMPTY; FIELD_LENGTH as usize];

    f
}
