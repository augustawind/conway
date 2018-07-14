extern crate clap;
extern crate conway;

use std::path::Path;

use clap::{App as CLI, Arg};
use conway::App;

static SAMPLE_DIR: &str = "./sample_patterns";
static SAMPLE_CHOICES: &[&str] = &["beacon", "glider", "oscillator", "toad"];

fn main() {
    let matches = CLI::new("Conway's Game of Life")
        .arg(
            Arg::with_name("file")
                .long("file")
                .takes_value(true)
                .required_unless("sample")
                .help("load a pattern from a file"),
        )
        .arg(
            Arg::with_name("sample")
                .long("sample")
                .takes_value(true)
                .required_unless("file")
                .conflicts_with("file")
                .possible_values(SAMPLE_CHOICES)
                .help("load a sample pattern"),
        )
        .get_matches();

    let app = if let Some(file) = matches.value_of("file") {
        let path = Path::new(file);
        App::from_path(path).unwrap()
    } else if let Some(sample) = matches.value_of("sample") {
        let path = Path::new(SAMPLE_DIR).join(sample);
        App::from_path(path).unwrap()
    } else {
        panic!("no pattern provided!");
    };

    app.run().unwrap();
}
