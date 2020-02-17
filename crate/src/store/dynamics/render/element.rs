#[derive(Debug)]
pub struct Canvas {
    width: u32,
    height: u32,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
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
}

impl Update for Canvas {
    fn set_width(&mut self, width: u32) {
        self.width = width
    }
    fn set_height(&mut self, height: u32) {
        self.height = height
    }
}

impl Get for Canvas {
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
}
