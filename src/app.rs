use crate::automata::Cell;

use std::default::Default;
use std::time::Duration;

use pancurses::{self, initscr, endwin, COLOR_PAIR, Input, Window, ALL_MOUSE_EVENTS};


pub type LifeGrid = Vec<Vec<Cell>>;

const FPS_DELAY: Duration = Duration::from_millis(100);


enum Message {
    Exit,
}

pub struct App {
    window: Window,
    simulating: bool,
    grid: LifeGrid,
}

impl Default for App {
    fn default() -> Self {
        let window = initscr();
        window.nodelay(true);
        window.keypad(true);

        pancurses::curs_set(0);
        pancurses::noecho();
        pancurses::mousemask(ALL_MOUSE_EVENTS, None);

        pancurses::start_color();
        pancurses::use_default_colors();

        // 1 -> UI Elements
        pancurses::init_pair(1, pancurses::COLOR_RED, -1);
        // 2 -> Cells (dead)
        pancurses::init_pair(2, 8, -1);

        let grid = vec![vec![Cell::default(); 1]; 1];
        let simulating = false;

        Self {window, simulating, grid}
    }
}

impl App {
    pub fn main(&mut self) {
        self.fit_term();
        loop {
            if let Some(msg) = self.handle_input() {
                match msg {
                    Message::Exit => break,
                }
            }

            if self.simulating {
                self.step();
            }

            self.render();

            std::thread::sleep(FPS_DELAY);
        }

        endwin();
    }


    //## Input Processing ##//

    fn handle_input(&mut self) -> Option<Message> {
        match self.window.getch() {
            Some(Input::Character('q')) => return Some(Message::Exit),
            Some(Input::Character('c')) => {
                self
                    .grid
                    .iter_mut()
                    .for_each(|line| {
                        line.iter_mut().for_each(|cell| cell.kill());
                    });
            },
            Some(Input::KeyEnter) | Some(Input::Character('\n')) | Some(Input::Character(' ')) => { 
                self.simulating = !self.simulating;
            },
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = pancurses::getmouse() {
                    let (x, y) = ((mouse_event.x - 1) as usize, (mouse_event.y - 1) as usize);

                    if let Some(row) = self.grid.get_mut(y) {
                        if let Some(col) = row.get_mut(x) {
                            col.toggle();
                        }
                    }
                }
            },
            _ => {},
        }
        None
    }


    //## Update ##//

    fn step(&mut self) {
        let rows = self.grid.len();
        let cols = self.grid.first().unwrap().len();
        let mut next_gen = vec![vec![Cell::default(); cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                let mut live: i8 = 0;

                for x in -1isize..=1 {
                    for y in -1isize..=1 {
                        let new_x = x + (j as isize);
                        let new_y = y + (i as isize);

                        if let Some(row) = self.grid.get(new_y as usize) {
                            if let Some(col) = row.get(new_x as usize) {
                                live += col.is_alive() as i8;
                            }
                        }
                    }
                }

                if self.grid[i][j].is_alive() {
                    live -= 1;
                }

                if self.grid[i][j].is_alive() && (live < 2 || live > 3) {
                    next_gen[i][j] = Cell::new(false);
                }
                else if !self.grid[i][j].is_alive() && live == 3 {
                    next_gen[i][j] = Cell::new(true);
                }
                else {
                    next_gen[i][j] = self.grid[i][j];
                }
            }
        }

        self.grid = next_gen;
    }


    //## Rendering ##//

    fn render(&self) {
        self.render_ui();
        self.render_grid();
        self.window.refresh();
    }

    pub fn render_ui(&self) {
        let rows = self.grid.len() as i32;
        let cols = self.grid
            .first()
            .expect("render_ui: empty row")
            .len() as i32;

        self.window.attron(pancurses::ColorPair(1));

        self.window.mvaddstr(0, 0, "╔");
        self.window.mvaddstr(0, 1, "═".repeat(cols as usize));
        self.window.mvaddstr(0, cols + 1, "╗");

        for i in 1..=(rows + 1) {
            self.window.mvaddstr(i, 0, "║");
            self.window.mvaddstr(i, cols + 1, "║");
        }

        self.window.mvaddstr(rows + 1, 0, "╚");
        self.window.mvaddstr(rows + 1, 1, "═".repeat(cols as usize));
        self.window.mvaddstr(rows + 1, cols + 1, "╝");

        self.window.attroff(pancurses::ColorPair(1));

        let mut spacing = 1;
        let ui_line = rows + 2;

        let pause_str = if self.simulating {
            "(SPACE) ⏸ Pause"
        }
        else {
            "(SPACE) ⏵ Play "
        };

        let clr_str = "(C) Clear";

        let quit_str = "(Q) Quit";


        self.window.mvaddstr(ui_line, spacing, pause_str);
        spacing += pause_str.len() as i32 + 1;

        self.window.mvaddstr(ui_line, spacing, clr_str);
        spacing += clr_str.len() as i32 + 1;

        self.window.mvaddstr(ui_line, spacing, quit_str);
    }

    fn render_grid(&self) {
        let (lim_i, lim_j) = self.window.get_max_yx();

        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let (i, j) = (i + 1, j + 1);
                if cell.is_alive() {
                    let (i, j) = (i as i32, j as i32);
                    if i <= lim_i && j <= lim_j {
                        self.window.mvaddstr(i, j, "■");
                    }
                }
                else {
                    let (i, j) = (i as i32, j as i32);
                    if i <= lim_i && j <= lim_j {
                        self.window.attron(COLOR_PAIR(2));
                        self.window.mvaddstr(i, j, "+");
                        self.window.attroff(COLOR_PAIR(2));
                    }
                }
            }
        }
    }


    //## Miscellaneous ##//

    fn fit_term(&mut self) {
        let (max_y, max_x) = self.window.get_max_yx();

        let (max_y, max_x) = (max_y as f64, max_x as f64);

        let x = (max_x * 0.8).round();
        let y = (x / (max_x / max_y)).round() as usize;
        let x = x as usize;

        for line in self.grid.iter_mut() {
            line.resize(x, Cell::default());
        }
        self.grid.resize(y, vec![Cell::default(); x]);

        let mut rect_iter = self
            .grid
            .iter()
            .map(|row| row.len());

        let rectangular = match rect_iter.next() {
            Some(first) => rect_iter.all(|x| x == first),
            _ => unreachable!(),
        };

        assert!(rectangular);
    }
}
