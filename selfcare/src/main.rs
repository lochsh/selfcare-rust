extern crate rand;

use rand::Rng;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn read_lines<P>(file_path: P) -> Vec<String> where P: AsRef<Path> {
    let file_path = file_path.as_ref();
    let file = match File::open(file_path) {
        Err(why) => panic!("Couldn't open file {}: {}",
                           file_path.display(),
                           why.description()),
        Ok(file) => file,
    };

    let buf = BufReader::new(file);
    buf.lines().map(|line| {
        match line {
            Ok(l) => l,
            Err(why) => panic!("Couldn't read file {}: {}",
                               file_path.display(),
                               why.description()),
        }
    }).collect()
}

fn random_pair<Foo, Bar, F, T>(foos: &[Foo],
                               bars: &[Bar], f: F) -> T 
                               where F: FnOnce(&Foo, &Bar) -> T {
    let mut rng = rand::thread_rng();
    let foo = rng.choose(foos).expect("No entries found for first part of random pair");
    let bar = rng.choose(bars).expect("No entries found for second part of random pair");
    f(foo, bar)
}

/// Print random inspiring message in format "You are a <adjective> <noun>".
///
/// Takes command line args `adj_file` and `noun_file`, respectively the paths
/// to the file containing the adjectives and the containing the nouns.
fn main() {
    let mut args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        args.push("adjectives.txt".to_string());
        args.push("nouns.txt".to_string());
    }
    let adj = read_lines(&args[1]);
    let nouns = read_lines(&args[2]);
    random_pair(&adj, &nouns, |adjective, noun| println!("You are a{} {}", adjective, noun));
}
