extern crate clap;
extern crate regex;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use clap::{Arg, App};
use log::{LogRecord, LogLevel, LogMetadata, LogLevelFilter};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

type CharCountMap = HashMap<char, u32>;

const ORIGIN_2_KEY_CHARS: &'static str = "(){}_\"@";
const NEW_2_KEY_CHARS: &'static str = "90[]2'";
const DEFAULT_FILE_NAME_FILTER: &'static str = r"^.*[.]java$";
const PATHS: &'static str = "paths";

pub fn init() {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Error);
        Box::new(SimpleLogger)
    }).unwrap();
}

fn main() {
    init();
    info!("Start up");
    let file_pattern_help = "RE pattern to filter files by name (default: ".to_string()
        + DEFAULT_FILE_NAME_FILTER + ")";
    let params = App::new("Keyboard Remap Estimator")
        .version("1.0")
        .author("Daneel S. Yaitskov")
        .arg(Arg::with_name("pattern")
             .short("c")
             .long("file_pattern")
             .help(file_pattern_help.as_str())
             .takes_value(true))
        .arg(Arg::with_name(PATHS)
             .index(1)
             .multiple(true))
        .get_matches();

    let pattern = match Regex::new(params.value_of("pattern")
                             .unwrap_or(DEFAULT_FILE_NAME_FILTER)) {
        Err(e) => panic!("file_pattern [{}] is not valid regex: {}",
                         params.value_of("pattern").unwrap(), e),
        Ok(pattern) => pattern
    };
    let mut count: CharCountMap = HashMap::new();
    for file_path in params.values_of(PATHS).unwrap() {
        count_chars(&pattern, &mut count, Path::new(file_path));
    }
    let origin_cost = calc_cost(ORIGIN_2_KEY_CHARS, &count);
    let new_cost = calc_cost(NEW_2_KEY_CHARS, &count);
    println!("User pressed {} keys, but he could press {} keys",
             origin_cost, new_cost);
    if new_cost < origin_cost {
        let diff = origin_cost - new_cost;
        let percent_diff = diff as f32 / (origin_cost as f32) * 100.0;
        println!("Profit is {} ({:.1}%) keys less.", diff, percent_diff)
    }
}

fn calc_cost(two_key_chars: &str, count: &CharCountMap) -> u32 {
    let mut cost = 0;
    for (c, n) in count {
        cost += n * if two_key_chars.chars().any(|a| a == *c) {
            2
        } else {
            1
        }
    }
    cost
}

fn count_chars(pattern: &Regex, count: &mut CharCountMap, path: &Path) {
    if path.is_file() && pattern.is_match(path.file_name().unwrap().to_str().unwrap()) {
        info!("Count chars in a regular file {}", path.display());
        let mut file = match File::open(path) {
            Err(why) => {
                error!("Couldn't open file {}: {}",
                       path.display(), why.description());
                return
            }
            Ok(file) => file
        };
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(why) => {
                error!("Couldn't read file [{}]: {}",
                       path.display(), why.description());
                return
            }
            Ok(_) => {}
        }
        for c in content.chars() {
            if count.contains_key(&c) {
                let n = count[&c];
                count.insert(c, n + 1);
            } else {
                count.insert(c, 1);
            }
        }
    } else if path.is_dir() {
        info!("Count chars in dir {}", path.display());
        for entry in path.read_dir().unwrap() {
            count_chars(pattern, count, entry.unwrap().path().as_path())
        }
    } else {
        info!("Ignore path {} due not a regular file nor dir", path.display())
    }
}
