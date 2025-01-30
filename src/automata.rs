use std::default::Default;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    alive: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            alive: false,
        }
    }
}

#[allow(dead_code)]
impl Cell {
    pub fn new(alive: bool) -> Self {
        Self{alive}
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn toggle(&mut self) {
        self.alive = !self.alive;
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }

    pub fn resurrect(&mut self) {
        self.alive = true;
    }
}
