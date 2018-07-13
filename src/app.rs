use cursive::traits::*;
use cursive::views::{Button, LinearLayout, TextView};
use cursive::Cursive;

use super::game::Game;

pub struct App {
    game: Game,
}

impl App {
    pub fn new(game: Game) -> App {
        App { game: game }
    }

    pub fn run(&mut self) {
        let mut siv: Cursive = Cursive::default();
        siv.add_layer(
            LinearLayout::horizontal()
                .child(
                    LinearLayout::vertical()
                        .child(Button::new("tick", |_| ()))
                        .child(Button::new("quit", |s| s.quit())),
                )
                .child(
                    TextView::new(self.game.draw())
                        .with_id("canvas")
                        .fixed_size((50, 20)),
                ),
        );
        siv.run();
    }
}
