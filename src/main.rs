extern crate clap;
extern crate conway;

use std::path::Path;
use std::time::Duration;

use clap::Arg;
use conway::{App, Config};

static SAMPLE_DIR: &str = "./sample_patterns";
static SAMPLE_CHOICES: &[&str] = &["beacon", "glider", "oscillator", "toad"];
static VIEW_CHOICES: &[&str] = &["moving", "fixed"];

fn main() {
    let matches = clap::App::new("Conway's Game of Life")
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
        .arg(
            Arg::with_name("raw")
                .long("raw")
                .help("stream raw output to stdout"),
        )
        .arg(
            Arg::with_name("delay")
                .long("delay")
                .default_value("500")
                .help("delay (ms) between ticks"),
        )
        .arg(
            Arg::with_name("view")
                .long("view")
                .default_value("moving")
                .possible_values(VIEW_CHOICES)
                .help("viewing mode"),
        )
        .get_matches();

    let mut config = Config::default();
    if let Ok(ms) = matches.value_of("delay").unwrap().parse() {
        config.stream_delay = Duration::from_millis(ms);
    }
    let view = matches.value_of("view").unwrap();

    let mut app = if let Some(file) = matches.value_of("file") {
        let path = Path::new(file);
        App::from_path(path).unwrap()
    } else if let Some(sample) = matches.value_of("sample") {
        let path = Path::new(SAMPLE_DIR).join(sample);
        App::from_path(path).unwrap()
    } else {
        panic!("no pattern provided!");
    };

    if matches.is_present("raw") {
        app.run_as_stream().unwrap();
    } else {
        app.run().unwrap();
    }
}
