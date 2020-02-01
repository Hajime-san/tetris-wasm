use crate::store;

#[derive(Debug)]
pub enum GameState {
    Start,
    Prepare,
    Playing,
    Continuing,
    Over,
}
