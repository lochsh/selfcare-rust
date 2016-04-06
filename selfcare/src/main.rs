extern crate rand;

use rand::Rng;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Takes file path, returns a vector of strings, each string a line in file.
///
/// # Arguments
/// * `file_path: &Path` - The path to the file to read
///
/// # Return value
/// Returns `Vec<String>` where each string is a line in the file
fn read_lines(file_path: &Path) -> Vec<String> { 
    let content = read_file(file_path);
    split_lines(content)
}

/// Takes file path, returns corresponding `File` object.
///
/// # Arguments
/// * `file_path: &Path` - The path to file to read
///
/// # Return value
/// Returns `File` corresponding to `file_path`
fn open_file(file_path: &Path) -> File {
    let display = file_path.display();
    let file = match File::open(file_path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                           Error::description(&why)),
        Ok(file) => file,
    };
    file
}

/// Takes `file path, returns string containing the file's content.
///
/// # Arguments
/// * `file_path: &Path` - The path of file to read
///
/// # Return value
/// Returns `String` containing contents of file
fn read_file(file_path: &Path) -> String {
    let mut file = open_file(file_path);
    let display = file_path.display();
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read file {}: {}", display,
                           Error::description(&why)),
        Ok(content) => content,
    };
    content
}

/// Split a string on newlines and return vector of resulting strings.
///
/// # Arguments
/// * `string: String` - string to be split
///
/// # Return value
/// Returns `Vec<String>`
fn split_lines(string: String) -> Vec<String> {
    string
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}

/// Chooses random adjective and noun from file, combines into message.
///
/// This function does not return the message, but prints it to stdout.
/// The message is in the format "You are a <adjective> <noun>."
///
/// # Arguments
/// * `adj: Vec<String>` - vector of adjectives
/// * `nouns: Vec<String` - vector of nouns
///
/// # Return value
/// ()
fn selfcare(adj: Vec<String>, nouns: Vec<String>) {
    let adjective = rand::thread_rng().choose(&adj).unwrap();
    let noun = rand::thread_rng().choose(&nouns).unwrap();
    println!("You are a{} {}", adjective, noun)
}
    

fn main() {
    let args: Vec<String> = env::args().collect();
    let adj_file = Path::new(&args[1]);
    let noun_file = Path::new(&args[2]);
    let adj = read_lines(adj_file);
    let nouns = read_lines(noun_file);
    selfcare(adj, nouns)
}
