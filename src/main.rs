extern crate conway;

use std::io;
use std::io::prelude::*;

use conway::Game;

fn main() {
    let mut game = Game::load().unwrap();
    let mut stdout = io::stdout();
    let hr = &{
        let (.., width, _) = game.rect.shape();
        "%".repeat(width as usize)
    };
    write!(stdout, "\n{}", hr).unwrap();
    for frame in game.iter() {
        write!(stdout, "\n{}{}", frame, hr).unwrap();
        stdout.flush().unwrap();
    }
}
