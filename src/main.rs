mod automata;
mod rendering;

use automata::Cell;
use rendering::{render_ui, render_grid};

use std::time::Duration;

use pancurses::{self, initscr, endwin, Input};

const MAX_COLS: usize = 100;
const MAX_ROWS: usize = 26;
const FPS_DELAY: Duration = Duration::from_millis(200);

fn main() {
    let window = initscr();
    window.nodelay(true);
    window.keypad(true);

    pancurses::curs_set(0);
    pancurses::noecho();
    pancurses::mousemask(pancurses::ALL_MOUSE_EVENTS, None);

    pancurses::start_color();
    pancurses::use_default_colors();

    // 1 -> UI Elements
    pancurses::init_pair(1, pancurses::COLOR_RED, -1);

    let mut life = [[Cell::default(); MAX_COLS]; MAX_ROWS];
    let mut simulating = false;

    loop {
        match window.getch() {
            Some(Input::Character('q')) => break,
            Some(Input::KeyEnter) | Some(Input::Character('\n')) | Some(Input::Character(' ')) => { 
                simulating = !simulating;
            },
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = pancurses::getmouse() {
                    let (x, y) = ((mouse_event.x - 1) as usize, (mouse_event.y - 1) as usize);

                    if x <= MAX_COLS && y <= MAX_ROWS {
                        life[y][x].toggle();
                    }
                }
            },
            _ => {},
        }

        if simulating {
            life = automata::next_generation(life);
        }

        render_ui(&window);
        render_grid(&window, &life);

        window.refresh();

        std::thread::sleep(FPS_DELAY);
    };
    endwin();
}
