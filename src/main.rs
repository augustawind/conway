extern crate conway;

use std::io;
use std::io::prelude::*;

use conway::Game;

const HR_CHAR: char = '%';

fn main() {
    let mut game = Game::load().unwrap();
    let mut stdout = io::stdout();
    let hr = {
        let (.., width, _) = game.rect.shape();
        HR_CHAR.to_string().repeat(width as usize)
    };
    write!(stdout, "\n").unwrap();
    for frame in game.iter() {
        write!(stdout, "{}\n{}", hr, frame).unwrap();
        stdout.flush().unwrap();
    }
}
