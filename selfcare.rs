use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;
use std::io::prelude::*;
use std::path::Path;

fn read_lines(file_path: &Path) -> Vec<String> { 
    let display = file_path.display();
    let file = match File::open(file_path) {
        Err(why) => panic!("couldn't open {}: {}",
                           display, Error::description(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect();
    lines
}

fn main() {
    let file_path = Path::new("../self-ca.re/adjectives.txt");
    println!("{}", read_lines(&file_path));
}
