use crate::app::LifeGrid;

use pancurses::{self, COLOR_PAIR, Window};


pub fn render_ui(window: &Window, life: &LifeGrid) {
    let rows = life.len();
    let cols = life
        .first()
        .expect("render_ui: empty row")
        .len();

    window.attron(pancurses::ColorPair(1));

    window.mvaddstr(0, 0, "╔");
    window.mvaddstr(0, 1, "═".repeat(cols));
    window.mvaddstr(0, (cols + 1) as i32, "╗");

    for i in 1..=(rows as i32 + 1) {
        window.mvaddstr(i, 0, "║");
        window.mvaddstr(i, (cols as i32) + 1, "║");
    }

    window.mvaddstr((rows + 1) as i32, 0, "╚");
    window.mvaddstr((rows + 1) as i32, 1, "═".repeat(cols as usize));
    window.mvaddstr((rows + 1) as i32, (cols + 1) as i32, "╝");

    window.attroff(pancurses::ColorPair(1));
}


pub fn render_grid(window: &Window, life: &LifeGrid) {
    let (lim_i, lim_j) = window.get_max_yx();

    for (i, row) in life.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let (i, j) = (i + 1, j + 1);
            if cell.is_alive() {
                let (i, j) = (i as i32, j as i32);
                if i <= lim_i && j <= lim_j {
                    window.mvaddstr(i, j, "■");
                }
            }
            else {
                let (i, j) = (i as i32, j as i32);
                if i <= lim_i && j <= lim_j {
                    window.attron(COLOR_PAIR(2));
                    window.mvaddstr(i, j, "+");
                    window.attroff(COLOR_PAIR(2));
                }
            }
        }
    }
}
