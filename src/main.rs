extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[macro_use]
extern crate lazy_static;

type CharCountMap = HashMap<char, u32>;

const ORIGIN_2_KEY_CHARS: &'static str = "(){}_\"@";
const NEW_2_KEY_CHARS: &'static str = "90[]2'";

fn main() {
    let pattern = Regex::new(r"^.*[.]java$").unwrap();
    let mut count: CharCountMap = HashMap::new();
    for file_path in env::args() {
        count_chars(&pattern, &mut count, Path::new(file_path.as_str()));
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
        println!("Count chars in a regular file {}", path.display());
        let mut file = match File::open(path) {
            Err(why) => panic!("Couldn't open file {}: {}",
                               path.display(), why.description()),
            Ok(file) => file
        };
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(why) => {
                println!("Couldn't read file [{}]: {}",
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
        println!("Count chars in dir {}", path.display());
        for entry in path.read_dir().unwrap() {
            count_chars(pattern, count, entry.unwrap().path().as_path())
        }
    } else {
        println!("Ignore path {} due not a regular file nor dir", path.display())
    }
}
