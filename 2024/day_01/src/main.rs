use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

fn problem_1(vec1: &Vec<i32>, vec2: &Vec<i32>) {
    let mut val = 0;
    for x in 0..vec1.len() {
        val += (vec1[x] - vec2[x]).abs();
    }
    println!("Problem 1: {val}");
}

fn problem_2(vec1: &Vec<i32>, vec2: &Vec<i32>) {
    let mut h = HashMap::new();
    for i in vec2 {
        *h.entry(i).or_insert(0) += 1;
    }
    
    let mut val = 0;
    for i in vec1 {
        val += i*(*h.entry(i).or_insert(0));
    }

    println!("Problem 2: {val}");

}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut text = String::new();
    reader.read_to_string(&mut text).unwrap();
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    text.split('\n').for_each( |x| {
        if x == "" {
           return  
        }
        let v: Vec<&str> = x.split_whitespace().collect();
        vec1.push(v[0].parse().unwrap());
        vec2.push(v[1].parse().unwrap());
    });
    
    vec1.sort();    
    vec2.sort();    
    
    problem_1(&vec1, &vec2);
    problem_2(&vec1, &vec2);

}
