use crate::store;

#[derive(Debug)]
pub enum GameState {
    Start,
    Prepare,
    Playing,
    Over,
}

trait Check {
    fn check_playable(field: &Vec<i32>, current_block: &store::statics::BlockPosition)
        -> GameState;
}

impl Check for GameState {
    fn check_playable(
        field: &Vec<i32>,
        current_block: &store::statics::BlockPosition,
    ) -> GameState {
        let mut flag = GameState::Playing;
        // check equality between fixed block and next queue
        for _ in field {
            for w in current_block {
                if field[*w as usize] == store::statics::Number::CURRENT
                    || field[*w as usize] != store::statics::Number::EMPTY
                {
                    flag = GameState::Over;
                }
            }
        }

        flag
    }
}
