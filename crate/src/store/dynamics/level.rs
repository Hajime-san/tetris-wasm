use math::round;

#[derive(Debug)]
pub struct Level {
    count: i32,
    completed_row: i32,
    speed: i32,
    difficulty: i32,
    score: i32,
    multiple_number: i32,
}


impl Default for Level {
    fn default() -> Self {
        Self {
            count: 0,
            completed_row: 0,
            speed: 1000,
            difficulty: 0,
            score: 0,
            multiple_number: 10,
        }
    }
}

impl Level {
    //
    // get field value methods
    //

    pub fn get_count(&self) -> &i32 {
        &self.count
    }
    pub fn get_completed_row(&self) -> &i32 {
        &self.completed_row
    }
    pub fn get_speed(&self) -> &i32 {
        &self.speed
    }
    pub fn get_difficulty(&self) -> &i32 {
        &self.difficulty
    }
    pub fn get_score(&self) -> &i32 {
        &self.score
    }
    pub fn get_multiple_number(&self) -> &i32 {
        &self.multiple_number
    }

    //
    // update field value methods
    //

    pub fn update_count(&mut self) {
        self.count += 1
    }

    pub fn update_completed_row(&mut self, length: &Vec<i32>) {
        let increment_number = length.len() as i32;

        self.completed_row += increment_number
    }

    pub fn update_level_and_speed(&mut self) {
        const N: i32 = 10;
        if self.completed_row == 0 {
            return;
        }

        if (self.completed_row % self.multiple_number) < 4
            && (self.completed_row / self.multiple_number) >= 1
        {
            if self.speed >= 100 {
                self.multiple_number += N;
                self.speed -= 100;
            }
            self.difficulty += 1
        }
    }

    pub fn update_score(&mut self, length: &Vec<i32>) {
        let completed_row = length.len() as i32;

        if completed_row == 0 {
            self.score += 10;
        }

        if completed_row < 1 {
            return;
        }

        // success to complete row
        let mut ratio = 1.0;
        let mut multple = 0.0;
        let increment = 50.0;
        for _ in length {
            multple += 0.2;
            ratio *= 1.2 + multple;
            let mut _score: f64 = round::floor(increment * ratio, 0) * 10.0;
            self.score += _score as i32;
        }
    }
}
