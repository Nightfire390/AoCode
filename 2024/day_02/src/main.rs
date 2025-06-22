use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    
    let mut text = String::new();

    reader.read_to_string(&mut text).unwrap();

    let mut val1 = 0;
    let mut val2 = 0;

    text.trim().split('\n').for_each( |x| {
        let mut vec: Vec<i32> = x.split_whitespace().map(|y| y.parse().unwrap()).collect();
        let mut a = 1;
        
        if vec[0] - vec[1] >  0 {
            for x in 0..(vec.len() - 1) {
                if (vec[x] - vec[x+1] > 3 || vec[x] - vec[x+1] <=0) {
                    a = 0;
                    break;
                }
            }
        } else {
            for x in 0..(vec.len() - 1) {
                if (vec[x+1] - vec[x] > 3 || vec[x+1] - vec[x] <=0) {
                    a = 0;
                    break;
                }
            }
        }

        val1 += a;
       
        let mut b = 1;
        if a == 0 {
            for i in 0..vec.len() {
                b = 1;
                let n = vec.remove(i);
                if vec[0] - vec[1] >  0 {
                    for x in 0..(vec.len() - 1) {
                        if (vec[x] - vec[x+1] > 3 || vec[x] - vec[x+1] <=0) {
                            b = 0;
                            break;
                        }
                    }
                } else {
                    for x in 0..(vec.len() - 1) {
                        if (vec[x+1] - vec[x] > 3 || vec[x+1] - vec[x] <=0) {
                            b = 0;
                            break;
                        }
                    }
                }
                vec.insert(i, n);
                if b == 1 {
                    break;
                }
            }
        }
        val2 += b;

    });

    println!("Problem 1: {val1}");
    println!("Problem 2: {val2}");
}
