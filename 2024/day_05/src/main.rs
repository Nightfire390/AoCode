use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut text = String::new();
    reader.read_to_string(&mut text).unwrap();

    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let mut h: HashMap<&str, Vec<&str>> = HashMap::new();
    text.trim().split('\n').for_each( |x| {
        if x.contains('|') {
            let v: Vec<&str> = x.trim().split('|').collect();
            h.entry(v[0]).and_modify( |e| e.push(v[1]) ).or_insert(vec![v[1]]);
        } else if x.contains(',') {
            let mut v: Vec<&str> = x.trim().split(',').collect();
            let mut con = true;

            'outer: for i in 0..v.len() {
                for j in 0..i {
                    if h.get(v[i]).unwrap().contains(&v[j]) {
                        con = false;
                        break 'outer;
                    }
                }
            }

            if con {
                val1 += v[v.len()/2].parse::<i32>().unwrap();
            } else {
                for i in 0..v.len() - 1 {
                    for j in 0..v.len() - 1 - i {
                        if !h.get(v[j]).unwrap().contains(&v[j+1]) {
                            let tmp = v[j];
                            v[j] = v[j+1];
                            v[j+1] = tmp;
                        }
                    }
                }
                val2 += v[v.len()/2].parse::<i32>().unwrap();
            }
        }
    });

    println!("Problem 1: {}", val1);
    println!("Problem 2: {}", val2);
}
