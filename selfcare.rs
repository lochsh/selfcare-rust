use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

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
        Ok(_) => print!("{}", content),
    };
    content
}

fn split_lines(string: &String) -> Vec<String> {
    string
        .lines()
        .map(ToOwned::to_owned)
        .collect()    
}
    

fn main() {
    let file_path = Path::new("../self-ca.re/adjectives.txt");
    println!("{:?}", read_lines(&file_path));
}
