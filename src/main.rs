mod app;
mod automata;
mod rendering;

use app::App;

fn main() {
    let mut life_application = App::default();
    life_application.main();
}
