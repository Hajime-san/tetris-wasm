use crate::store;

#[derive(Debug)]
pub struct Delete {
    pub single_rows: Vec<Vec<i32>>,
    pub complete_row_numbers: Vec<i32>,
    pub remain_rows: Vec<Vec<i32>>,
    pub remain_row_numbers: Vec<i32>,
}

trait Update {
    fn create_single_rows(&mut self, field: &Vec<i32>);
    fn delete_row(&mut self, mut field: Vec<i32>);
    fn drop_row(&mut self, mut field: Vec<i32>);
}

impl Default for Delete {
    fn default() -> Self {
        Self {
            single_rows: vec![],
            complete_row_numbers: vec![],
            remain_rows: vec![],
            remain_row_numbers: vec![],
        }
    }
}

impl Update for Delete {
    fn create_single_rows(&mut self, field: &Vec<i32>) {
        // creat vector on each rows
        let mut start = 0;
        let mut end = store::statics::Number::ROW;

        for _ in 0..store::statics::Number::COLUMN {
            let row: Vec<i32> = (field[start as usize]..field[end as usize]).collect();
            start += store::statics::Number::ROW;
            end += store::statics::Number::ROW;
            self.single_rows.push(row);
        }
    }

    fn delete_row(&mut self, mut field: Vec<i32>) {
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
            field[v as usize] = store::statics::Number::EMPTY;
        }
    }

    fn drop_row(&mut self, mut field: Vec<i32>) {
        // first, drop remainRow

        if self.remain_row_numbers.len() > 0 {
            let mut reverse_num: Vec<i32> = self.remain_row_numbers.iter().rev().cloned().collect();
            let reverse_rows: Vec<Vec<i32>> = self.remain_rows.iter().rev().cloned().collect();

            for _ in 0..store::statics::Number::COLUMN as usize {
                // loop
                for (i, v) in reverse_num.iter().enumerate() {
                    let start = (v * store::statics::Number::ROW) + store::statics::Number::ROW;

                    if start >= field.len() as i32 {
                        return;
                    }

                    let check: Vec<i32> = field
                        .iter()
                        .filter(|&k| k >= &start && k <= &(start + store::statics::Number::ROW - 1))
                        .cloned()
                        .collect();

                    if check
                        .iter()
                        .all(|&x| field[x as usize] == store::statics::Number::EMPTY)
                    {
                        for j in 0..store::statics::Number::ROW {
                            let value = reverse_rows[i as usize][j as usize];

                            field[(start + j) as usize] = value;
                            field[((start - store::statics::Number::ROW) + j) as usize] =
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
