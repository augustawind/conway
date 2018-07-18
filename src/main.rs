extern crate conway;

use conway::App;

fn main() {
    let mut app = App::load().unwrap();
    app.run().unwrap();
}
