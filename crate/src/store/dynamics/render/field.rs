use math::round;

use crate::func;
use crate::store;

#[derive(Debug)]
pub struct Field {
    width: u32,
    height: u32,
    horizon: f64,
    vertical: f64,
    render_boundary: f64,
    step: f64,
    queue_step: f64,
    rect: Rect
}

#[derive(Debug)]
struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
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
            width: 0,
            height: 0,
            horizon: 270.0,
            vertical: 378.0,
            render_boundary: 7.0,
            step: 27.0,
            queue_step: 20.0,
            rect: Rect {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0
            }
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

pub trait Update {
    fn set_width(&mut self, width: u32);
    fn set_height(&mut self, height: u32);
}
pub trait Get {
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_step(&self) -> f64;
    fn get_boundary(&self, step_time: &i32) -> f64;
    fn get_render_boundary(&self) -> f64;
    fn get_horizon_boundary(&self) -> f64;
    fn get_vertical_boundary(&self) -> f64;
    fn get_rect_x(&self, position: &usize) -> f64;
    fn get_rect_y(&self, position: &usize) -> f64;
    fn get_rect_w(&self) -> f64;
    fn get_rect_h(&self) -> f64;
}

impl Update for Field {
    fn set_width(&mut self, width: u32) {
        self.width = width
    }
    fn set_height(&mut self, height: u32) {
        self.height = height
    }
}

impl Get for Field {
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
    fn get_step(&self) -> f64 {
        self.step
    }
    fn get_boundary(&self, step_time: &i32) -> f64 {
        self.step * (*step_time as f64)
    }
    fn get_render_boundary(&self) -> f64 {
        self.render_boundary
    }
    fn get_horizon_boundary(&self) -> f64 {
        self.step * (*&store::statics::Number::ROW as f64)
    }
    fn get_vertical_boundary(&self) -> f64 {
        self.step * (*&store::statics::Number::COLUMN as f64)
    }
    fn get_rect_x(&self, position: &usize) -> f64 {
        (((func::fix_digit(*position as i32) * self.step as i32) as f64) + (self.render_boundary + 1.0))
    }
    fn get_rect_y(&self, position: &usize) -> f64 {
        (round::floor((*position as i32 / store::statics::Number::ROW) as f64, 0) as f64)
                * self.step
                + (self.render_boundary + 1.0)
    }
    fn get_rect_w(&self) -> f64 {
        self.step - 2.0
    }
    fn get_rect_h(&self) -> f64 {
        self.step - 2.0
    }
}
