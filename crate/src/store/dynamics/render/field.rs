#[derive(Debug)]
pub struct Field {
    pub horizon: f64,
    pub vertical: f64,
    pub standard: f64,
    pub step: f64,
    pub queue_step: f64,
}

pub struct Text {
    font: &'static str,
    font_size: &'static str,
    font_size2: &'static str,
    next: &'static str,
    line: &'static str,
    score: &'static str,
    play: &'static str,
    game_over: &'static str,
    replay: &'static str,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            horizon: 270.0,
            vertical: 378.0,
            standard: 7.0,
            step: 27.0,
            queue_step: 20.0,
        }
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            font: "Osaka",
            font_size: "16px ",
            font_size2: "18px ",
            next: "Next",
            line: "Line",
            score: "Score",
            play: "Play ?",
            game_over: "Game Over",
            replay: "Replay ?",
        }
    }
}
