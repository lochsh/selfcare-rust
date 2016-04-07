extern crate rand;

use rand::Rng;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_lines<P>(file_path: P) -> Vec<String> where P: AsRef<Path> { 
    let content = read_file(file_path);
    split_lines(&content)
}

fn open_file<P>(file_path: P) -> File where P: AsRef<Path> {
    let file_path = file_path.as_ref();
    match File::open(file_path) {
        Err(why) => panic!("Couldn't open file {}: {}",
                           file_path.display(),
                           Error::description(&why)),
        Ok(file) => file,
    }
}

fn read_file<P>(file_path: P) -> String where P: AsRef<Path> {
    let file_path = file_path.as_ref();
    let mut file = open_file(file_path);
    let mut content = String::new();

    if let Err(why) = file.read_to_string(&mut content) {
        panic!("Couldn't read file {}: {}",
               file_path.display(),
               why.description());
    }
    content
}

fn split_lines(string: &str) -> Vec<String> {
    string
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}

/// Chooses random adjective and noun from file, combines into message.
///
/// This function does not return the message, but prints it to stdout.
/// The message is in the format "You are a <adjective> <noun>."
fn selfcare<S>(adj: &[S], nouns: &[S]) where S: std::fmt::Display {
    let mut rng = rand::thread_rng();
    let adjective = rng.choose(adj).expect("No adjectives found!");
    let noun = rng.choose(nouns).expect("No nouns found!");
    println!("You are a{} {}", adjective, noun)
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
    selfcare(&adj, &nouns)
}
