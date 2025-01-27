mod automata;

use automata::Cell;

use std::time::Duration;

use pancurses::{self, initscr, endwin, Input};

const MAX_COLS: usize = 300;
const MAX_ROWS: usize = 300;
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

    let mut life = [[Cell::default(); MAX_COLS]; MAX_ROWS];
    let mut simulating = false;

    loop {
        match window.getch() {
            Some(Input::Character('q')) => break,
            Some(Input::KeyEnter) | Some(Input::Character('\n')) | Some(Input::Character(' ')) => { 
                simulating = !simulating ;
            },
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = pancurses::getmouse() {
                    life[mouse_event.y as usize][mouse_event.x as usize].resurrect();
                }
            },
            _ => {},
        }

        if simulating {
            life = automata::next_generation(life);
        }

        let (lim_i, lim_j) = window.get_max_yx();

        for (i, row) in life.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell.is_alive() {
                    let (i, j) = (i as i32, j as i32);
                    if i <= lim_i && j <= lim_j {
                        window.mvaddch(i, j, '#');
                    }
                }
                else {
                    let (i, j) = (i as i32, j as i32);
                    if i <= lim_i && j <= lim_j {
                        window.mvaddch(i, j, ' ');
                    }
                }
            }
        }

        window.refresh();

        std::thread::sleep(FPS_DELAY);
    };
    endwin();
}
