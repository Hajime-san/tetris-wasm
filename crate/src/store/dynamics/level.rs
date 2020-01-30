use crate::store;

#[derive(Debug)]
pub struct Level {
    pub count: i32,
    pub completed_row: i32,
    pub speed: i32,
    pub difficulty: i32,
    pub score: i32,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            count: 1,
            completed_row: 0,
            speed: 1000,
            difficulty: 0,
            score: 0,
        }
    }
}
