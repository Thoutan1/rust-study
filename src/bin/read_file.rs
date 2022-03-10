use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("file/test.txt").expect("Cannot open this file");

    let mut content =  String::new();
    file.read_to_string(&mut content).expect("Cannot read this file");

    println!("File content:\n\n{}", content);
}