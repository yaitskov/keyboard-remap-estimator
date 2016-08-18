use std::collections::HashMap;
use std::env;
use std::path::Path;

const ORIGIN_2_KEY_CHARS: &'static str = "(){}_\"@";
const NEW_2_KEY_CHARS: &'static str = "90[]2'";

fn main() {
    let mut count: HashMap<char, i32> = HashMap::new();
    for file_path in env::args() {
        analyze(&mut count, Path::new(file_path.as_str()));
    }
}

fn analyze(count: &mut HashMap<char, i32>, path: &Path) {
    if path.is_file() {
        println!("Analyze a regular file {}", path.display());
    } else if path.is_dir() {
        println!("Analyze dir {}", path.display());
        for entry in path.read_dir().unwrap() {
            analyze(count, entry.unwrap().path().as_path())
        }
    } else {
        println!("Ignore path {} due not a regular file nor dir", path.display())
    }
}
