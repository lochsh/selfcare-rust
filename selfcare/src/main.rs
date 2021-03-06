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
                           file_path.display(), why.description()),
        Ok(file) => file,
    };

    BufReader::new(file).lines().map(|line| {
        match line {
            Ok(l) => l,
            Err(why) => panic!("Couldn't read file {}: {}",
                               file_path.display(), why.description()),
        }
    }).collect()
}


fn random_pair<'a, T>(foos: &'a[T], bars: &'a[T]) -> (&'a T, &'a T) {
    let mut rng = rand::thread_rng();
    let foo = rng.choose(foos).expect("No entries found for first part of random pair");
    let bar = rng.choose(bars).expect("No entries found for second part of random pair");
    (foo, bar)
}


/// Print random inspiring message in format "You are a <adjective> <noun>".
///
/// Takes two command line args: the paths to the file containing the
/// adjectives and the file containing the nouns.
fn main() {
    let args: Vec<String> = env::args().collect();

    let adj_filename = args.get(1).map_or("adjectives.txt", |f| f);
    let noun_filename = args.get(2).map_or("nouns.txt", |f| f);

    let adj = read_lines(adj_filename);
    let nouns = read_lines(noun_filename);
    let (chosen_adj, chosen_noun) = random_pair(&adj, &nouns);
    println!("You are a{} {}", chosen_adj, chosen_noun);
}
