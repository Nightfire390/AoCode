use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut text = String::new();
    reader.read_to_string(&mut text);

    let mat: Vec<Vec<char>> = text.trim().split('\n').map( |x| x.trim().chars().collect() ).collect();
    println!("{:?}", mat);

}
