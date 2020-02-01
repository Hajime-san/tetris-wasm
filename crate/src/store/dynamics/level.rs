use math::round;

use crate::store;

#[derive(Debug)]
pub struct Level {
    count: i32,
    completed_row: i32,
    speed: i32,
    difficulty: i32,
    score: i32,
    multiple_number: i32,
}

trait Update {
    fn update_count(&mut self, game_state: &store::dynamics::manager::GameState);
    fn update_completed_row(
        &mut self,
        length: &Vec<i32>,
        game_state: &store::dynamics::manager::GameState,
    );
    fn update_level_and_speed(&mut self, game_state: &store::dynamics::manager::GameState);
    fn update_score(&mut self, length: &Vec<i32>, game_state: &store::dynamics::manager::GameState);
}

trait Get {
    fn get_value(&self) -> Self;
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

impl Update for Level {
    fn update_count(&mut self, game_state: &store::dynamics::manager::GameState) {
        match game_state {
            store::dynamics::manager::GameState::Continuing => self.count += 1,
            _ => self.count = 0,
        }
    }

    fn update_completed_row(
        &mut self,
        length: &Vec<i32>,
        game_state: &store::dynamics::manager::GameState,
    ) {
        let increment_number = length.len() as i32;

        match game_state {
            store::dynamics::manager::GameState::Over => self.completed_row = 0,
            _ => self.completed_row += increment_number,
        }
    }

    fn update_level_and_speed(&mut self, game_state: &store::dynamics::manager::GameState) {
        let mut level = || {
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
        };

        match game_state {
            store::dynamics::manager::GameState::Over => {
                self.difficulty = 0;
                self.speed = 1000;
            }
            _ => level(),
        }
    }

    fn update_score(
        &mut self,
        length: &Vec<i32>,
        game_state: &store::dynamics::manager::GameState,
    ) {
        let completed_row = length.len() as i32;

        let mut score = || {
            // fail to complete row
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
            for v in length {
                multple += 0.2;
                ratio *= 1.2 + multple;
                let mut _score: f64 = round::floor(increment * ratio, 0) * 10.0;
                self.score += _score as i32;
            }
        };

        match game_state {
            store::dynamics::manager::GameState::Over => self.score = 0,
            _ => score(),
        }
    }
}

impl Get for Level {
    fn get_value(&self) -> Self {
        Self {
            count: self.count,
            completed_row: self.completed_row,
            speed: self.speed,
            difficulty: self.difficulty,
            score: self.score,
            multiple_number: self.multiple_number,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::store;

    use super::Level;

    use super::Update;

    use super::Get;

    use store::dynamics::manager::GameState;

    #[test]
    fn some_test() {
        let mut options: Level = Default::default();

        let mut game_flag = store::dynamics::manager::GameState::Continuing;

        let v = vec![0; 1];

        options.update_completed_row(&v, &game_flag);

        options.update_score(&v, &game_flag);

        let aa = options.get_value().speed;

        println!("{:?}", aa);

        println!("{:?}", options.get_value().score);
    }
}
