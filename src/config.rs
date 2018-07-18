use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

use clap;
use clap::{Arg, ArgMatches};

use super::{App, AppResult, Game, Grid};

static SAMPLE_DIR: &str = "./sample_patterns";

static SAMPLE_CHOICES: &[&str] = &["beacon", "glider", "oscillator", "toad"];
static VIEW_CHOICES: &[&str] = &["moving", "fixed"];

fn parse_args<'a, I, T>(args: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    fn validate_single_char(s: String) -> Result<(), String> {
        if s.len() != 1 {
            return Err("must be a single character".to_string());
        }
        Ok(())
    }

    clap::App::new("Conway's Game of Life")
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
                .conflicts_with("file")
                .possible_values(SAMPLE_CHOICES)
                .default_value("glider")
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
                .help("delay (ms) between ticks")
                .validator(|s| {
                    if s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()) {
                        return Err("must be a valid number of milliseconds".to_string());
                    }
                    Ok(())
                }),
        )
        .arg(
            Arg::with_name("view")
                .long("view")
                .default_value("moving")
                .possible_values(VIEW_CHOICES)
                .help("viewing mode"),
        )
        .arg(
            Arg::with_name("live-char")
                .long("live-char")
                .default_value("x")
                .help("char to represent living cells")
                .validator(validate_single_char),
        )
        .arg(
            Arg::with_name("dead-char")
                .long("dead-char")
                .default_value(".")
                .help("char to represent dead cells")
                .validator(validate_single_char),
        )
        .get_matches_from(args)
}

#[derive(Default)]
pub struct Config {
    pub pattern: String,
    pub raw: bool,
    pub stream_delay: Duration,
    pub view: String,
    pub live_char: char,
    pub dead_char: char,
}

impl Config {
    pub fn load() -> AppResult<Config> {
        let matches = parse_args(env::args_os());
        let mut config = Config::default();

        if let Ok(ms) = matches.value_of("delay").unwrap().parse() {
            config.stream_delay = Duration::from_millis(ms);
        }
        config.raw = matches.is_present("raw");
        config.view = matches.value_of("view").unwrap().to_string();
        config.live_char = matches
            .value_of("live-char")
            .unwrap()
            .chars()
            .next()
            .unwrap();
        config.dead_char = matches
            .value_of("dead-char")
            .unwrap()
            .chars()
            .next()
            .unwrap();

        config.pattern = {
            let path = if let Some(file) = matches.value_of("file") {
                Path::new(file).to_path_buf()
            } else {
                let file = matches.value_of("sample").unwrap();
                Path::new(SAMPLE_DIR).join(file)
            };
            let mut f = File::open(path)?;
            let mut pattern = String::new();
            f.read_to_string(&mut pattern)?;
            pattern
        };

        Ok(config)
    }
}
