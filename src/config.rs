use std::default::Default;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use clap::{App, Arg, ArgGroup, ArgMatches};

use grid::View;
use AppResult;

static SAMPLE_DIR: &str = "./sample_patterns";

static SAMPLE_CHOICES: &[&str] = &["beacon", "glider", "oscillator", "toad"];
static VIEW_CHOICES: &[&str] = &["centered", "fixed", "follow"];

const CHAR_ALIVE: char = 'x';
const CHAR_DEAD: char = '.';

fn parse_args<'a, I, T>(args: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    App::new("Conway's Game of Life")
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
                .help("delay (ms) between ticks"),
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
                .help("char to represent living cells"),
            Arg::with_name("dead-char")
                .long("dead-char")
                .help("char to represent dead cells"),
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

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub raw: bool,
    pub stream_delay: Duration,
    pub view: View,
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

        config.view = matches.value_of("view").unwrap().parse()?;

        config.char_alive = matches
            .value_of("live-char")
            .map_or(Ok(CHAR_ALIVE), FromStr::from_str)?;
        config.char_dead = matches
            .value_of("dead-char")
            .map_or(Ok(CHAR_DEAD), FromStr::from_str)?;

        config.min_width = matches.value_of("min-width").unwrap().parse()?;
        config.min_height = matches.value_of("min-width").unwrap().parse()?;
        config.max_width = matches.value_of("max-width").unwrap().parse()?;
        config.max_height = matches.value_of("max-width").unwrap().parse()?;

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

impl Default for Config {
    fn default() -> Self {
        Config {
            pattern: Default::default(),
            raw: Default::default(),
            stream_delay: Duration::from_millis(500),
            view: View::Centered,
            char_alive: CHAR_ALIVE,
            char_dead: CHAR_DEAD,
            min_width: 10,
            min_height: 10,
            max_width: 30,
            max_height: 30,
        }
    }
}
