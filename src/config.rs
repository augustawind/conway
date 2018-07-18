use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

use clap;
use clap::{Arg, ArgGroup, ArgMatches};

use super::{AppError, AppResult};

static SAMPLE_DIR: &str = "./sample_patterns";

static SAMPLE_CHOICES: &[&str] = &["beacon", "glider", "oscillator", "toad"];
static VIEW_CHOICES: &[&str] = &["centered", "fixed", "follow"];

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
                .help("load a pattern from a file"),
        )
        .arg(
            Arg::with_name("sample")
                .long("sample")
                .possible_values(SAMPLE_CHOICES)
                .default_value("glider")
                .help("load a sample pattern"),
        )
        .group(
            ArgGroup::with_name("source")
                .args(&["file", "sample"])
                .required(true),
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
                .default_value("centered")
                .possible_values(VIEW_CHOICES)
                .help("viewing mode"),
        )
        .args(&[
            Arg::with_name("live-char")
                .long("live-char")
                .default_value("x")
                .help("char to represent living cells")
                .validator(validate_single_char),
            Arg::with_name("dead-char")
                .long("dead-char")
                .default_value(".")
                .help("char to represent dead cells")
                .validator(validate_single_char),
        ])
        .args(&[
            Arg::with_name("min-width")
                .long("min-width")
                .default_value("0")
                .help("minimum grid width displayed"),
            Arg::with_name("min-height")
                .long("min-height")
                .default_value("0")
                .help("minimum grid height displayed"),
            Arg::with_name("max-width")
                .long("max-width")
                .default_value("0")
                .help("maximum grid width displayed"),
            Arg::with_name("max-height")
                .long("max-height")
                .default_value("0")
                .help("maximum grid height displayed"),
        ])
        .get_matches_from(args)
}

#[derive(Default)]
pub struct Config {
    pub pattern: String,
    pub raw: bool,
    pub stream_delay: Duration,
    pub view: String,
    pub char_alive: char,
    pub char_dead: char,
    pub min_width: u64,
    pub min_height: u64,
    pub max_width: u64,
    pub max_height: u64,
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
        config.char_alive = matches
            .value_of("live-char")
            .unwrap()
            .chars()
            .next()
            .unwrap();
        config.char_dead = matches
            .value_of("dead-char")
            .unwrap()
            .chars()
            .next()
            .unwrap();

        let e = AppError::new("must be a valid integer");
        config.min_width = matches
            .value_of("min-width")
            .unwrap()
            .parse()
            .map_err(|_| e.clone())?;
        config.min_height = matches
            .value_of("min-width")
            .unwrap()
            .parse()
            .map_err(|_| e.clone())?;
        config.max_width = matches
            .value_of("max-width")
            .unwrap()
            .parse()
            .map_err(|_| e.clone())?;
        config.max_height = matches
            .value_of("max-width")
            .unwrap()
            .parse()
            .map_err(|_| e.clone())?;

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
