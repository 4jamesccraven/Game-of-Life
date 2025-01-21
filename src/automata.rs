use crate::{MAX_ROWS, MAX_COLS};

use std::default::Default;

pub type LifeGrid = [[Cell; MAX_COLS]; MAX_ROWS];

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1,  0),
    (-1,  1),
    ( 0, -1),
    ( 0,  1),
    ( 1, -1),
    ( 1,  0),
    ( 1,  1),
];

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

impl Cell {
    pub fn new(alive: bool) -> Self {
        Self{alive}
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    #[allow(dead_code)]
    pub fn toggle(&mut self) {
        self.alive = !self.alive;
    }

    #[allow(dead_code)]
    pub fn kill(&mut self) {
        self.alive = false;
    }

    #[allow(dead_code)]
    pub fn resurrect(&mut self) {
        self.alive = true;
    }
}

fn bounds_check(x: isize, y: isize) -> bool {
    (0 <= x && x < MAX_ROWS as isize) && (0 <= y && y < MAX_COLS as isize)
}

pub fn next_generation(life: LifeGrid) -> LifeGrid {
    let mut next_gen = [[Cell::default(); MAX_COLS]; MAX_ROWS];
    for i in 0..MAX_ROWS {
        for j in 0..MAX_COLS {
            let live = DIRS
                .iter()
                .filter(|&(dy, dx)| {
                    let (x, y) = (i as isize + dy, j as isize + dx);
                    bounds_check(x, y) && life[y as usize][x as usize].is_alive()
                })
                .count();

            if life[i][j].is_alive() && (live < 2 || live > 3) {
                next_gen[i][j] = Cell::new(false);
            }
            else if !life[i][j].is_alive() && live == 3 {
                next_gen[i][j] = Cell::new(true);
            }
            else {
                next_gen[i][j] = life[i][j];
            }
        }
    }

    next_gen
}
