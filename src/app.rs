use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use cursive::traits::*;
use cursive::views::{Button, LinearLayout, TextView};
use cursive::Cursive;

use super::{Game, Grid};

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn start_pattern(&self, path: &str) -> Result<(), String> {
        let mut f = File::open(path).map_err(|e| e.to_string())?;
        let mut pattern = String::new();
        f.read_to_string(&mut pattern).map_err(|e| e.to_string())?;
        let grid: Grid = pattern.parse()?;
        let game = Game::new(grid);
        for output in game {
            println!("{}", output);
            thread::sleep(Duration::from_millis(1));
        }
        Ok(())
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
                .child(TextView::new("").with_id("canvas").fixed_size((50, 20))),
        );
        siv.run();
    }
}
