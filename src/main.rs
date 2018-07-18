extern crate conway;

use std::io;

use conway::App;

fn main() {
    let mut app = App::load().unwrap();
    let mut stdout = io::stdout();
    app.run(&mut stdout).unwrap();
}
