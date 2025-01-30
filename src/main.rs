mod app;
mod automata;

use app::App;

fn main() {
    let mut life_application = App::default();
    life_application.main();
}
