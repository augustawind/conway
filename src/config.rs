use std::default::Default;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use clap::ArgMatches;

use game::View;
use AppResult;

static SAMPLE_DIR: &str = "./sample_patterns";
static SAMPLE_CHOICES: &[&str] = &["beacon", "glider", "blinker", "toad"];
static VIEW_CHOICES: &[&str] = &["centered", "fixed", "follow"];
pub const CHAR_ALIVE: char = '#';
pub const CHAR_DEAD: char = '-';

fn parse_args<'a, I, T>(args: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    clap_app!(("Conway's Game of Life") =>
        (version: "0.1")
        (author: "Dustin Rohde <dustin.rohde@gmail.com>")
        (about: "A shell utility for running Conway's Game of Life simulations.")
        (@group source +required =>
            (@arg file: -F --file +takes_value "load a pattern from a file")
            (@arg sample: -S --sample
                possible_values(SAMPLE_CHOICES) default_value[glider]
                "load a sample pattern")
        )
        (@arg delay: -d --delay default_value("500") "delay (ms) between ticks")
        (@arg live_char: -o --("live-char") +takes_value "character used to render live cells")
        (@arg dead_char: -x --("dead-char") +takes_value "character used to render dead cells")
        (@arg view: -v --view possible_values(VIEW_CHOICES) default_value[fixed]
            "viewing mode")
        (@arg min_width: -W --("min-width") default_value("0") "minimum width of output")
        (@arg min_height: -H --("min-height") default_value("0") "minimum height of output")
        (@arg width: -w --width default_value("20") "viewport width")
        (@arg height: -h --height default_value("10") "viewport height")
    ).get_matches_from(args)
}

#[derive(Debug)]
pub struct ConfigReader {
    pub settings: Settings,
    pub pattern: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub tick_delay: Duration,
    pub view: View,

    pub width: u64,
    pub height: u64,
    pub min_width: u64,
    pub min_height: u64,

    pub char_alive: char,
    pub char_dead: char,
}

impl ConfigReader {
    pub fn from_env() -> AppResult<ConfigReader> {
        ConfigReader::from_args(env::args_os())
    }

    pub fn from_args<I, T>(args: I) -> AppResult<ConfigReader>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches = parse_args(args);

        let conf = ConfigReader {
            settings: Settings {
                tick_delay: Duration::from_millis(matches.value_of("delay").unwrap().parse()?),

                view: matches.value_of("view").unwrap().parse()?,

                min_width: matches.value_of("min_width").unwrap().parse()?,
                min_height: matches.value_of("min_width").unwrap().parse()?,
                width: matches.value_of("width").unwrap().parse()?,
                height: matches.value_of("height").unwrap().parse()?,

                char_alive: matches
                    .value_of("live_char")
                    .map_or(Ok(CHAR_ALIVE), FromStr::from_str)?,
                char_dead: matches
                    .value_of("dead_char")
                    .map_or(Ok(CHAR_DEAD), FromStr::from_str)?,
            },
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
        };

        Ok(conf)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            tick_delay: Duration::from_millis(500),
            view: View::Centered,
            min_width: 10,
            min_height: 10,
            width: 10,
            height: 10,
            char_alive: CHAR_ALIVE,
            char_dead: CHAR_DEAD,
        }
    }
}
