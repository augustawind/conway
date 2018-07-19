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
pub struct ConfigSet {
    pub grid: GridConfig,
    pub game: GameConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameConfig {
    pub raw_mode: bool,
    pub tick_delay: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridConfig {
    pub pattern: String,
    pub view: View,
    pub char_alive: char,
    pub char_dead: char,
    pub min_width: u64,
    pub min_height: u64,
    pub max_width: u64,
    pub max_height: u64,
}

impl ConfigSet {
    pub fn load() -> AppResult<ConfigSet> {
        let matches = parse_args(env::args_os());

        let conf = ConfigSet {
            game: GameConfig {
                raw_mode: matches.is_present("raw"),
                tick_delay: Duration::from_millis(matches.value_of("delay").unwrap().parse()?),
            },
            grid: GridConfig {
                view: matches.value_of("view").unwrap().parse()?,

                char_alive: matches
                    .value_of("live-char")
                    .map_or(Ok(CHAR_ALIVE), FromStr::from_str)?,
                char_dead: matches
                    .value_of("dead-char")
                    .map_or(Ok(CHAR_DEAD), FromStr::from_str)?,

                min_width: matches.value_of("min-width").unwrap().parse()?,
                min_height: matches.value_of("min-width").unwrap().parse()?,
                max_width: matches.value_of("max-width").unwrap().parse()?,
                max_height: matches.value_of("max-width").unwrap().parse()?,

                pattern: {
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
                },
            },
        };

        Ok(conf)
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            raw_mode: false,
            tick_delay: Duration::from_millis(500),
        }
    }
}

impl Default for GridConfig {
    fn default() -> Self {
        GridConfig {
            pattern: Default::default(),
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
