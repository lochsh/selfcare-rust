extern crate rand;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use rand::Rng;

fn read_lines(file_path: &Path) -> Vec<String> { 
    let file = open_file(file_path);
    let content = read_file(&file);
    let lines = split_lines(&content);
    lines

}

fn open_file(file_path: &Path) -> File {
    let display = file_path.display();
    let mut file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                           Error::description(&why)),
        Ok(file) => file,
    };
    file
}

fn read_file(mut file: &File) -> String {
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read file: {}",
                           Error::description(&why)),
        Ok(content) => content,
    };
    content
}

fn split_lines(string: &String) -> Vec<String> {
    string
        .lines()
        .map(ToOwned::to_owned)
        .collect()    
}

fn selfcare(adj: Vec<String>, nouns: Vec<String>) {
    let num_adj = adj.len();
    let num_noun = nouns.len();
    let ind_adj = rand::thread_rng().gen_range(0, num_adj);
    let ind_noun = rand::thread_rng().gen_range(0, num_noun);
    println!("You are a{} {}", adj[ind_adj], nouns[ind_noun])
}
    

fn main() {
    let adj_file = Path::new("adjectives.txt");
    let noun_file = Path::new("nouns.txt");
    let adj = read_lines(adj_file);
    let nouns = read_lines(noun_file);
    selfcare(adj, nouns)
}
