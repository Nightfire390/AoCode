use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut text = String::new();
    reader.read_to_string(&mut text).unwrap();

    let mut index = 0;
    let text = text.trim();
    let len = text.len();
    let mut val1 = 0;
    let mut val2 = 0;
    let mut con = true;

    'out: while index < len - 8 {

        if &text[index..(index+4)] == "do()" { 
            con = true;
            index += 4;
            continue;
        }

        if &text[index..(index+7)] == "don't()" { 
            con = false;
            index += 7;
            continue;
        }

        if &text[index..(index+4)] == "mul(" && (&text[index+4..index+12]).contains(")") && (&text[index+4..index+12]).contains(",") {
            let a = (&text[index+4..index+12]).find(')').unwrap();
            let arr: Vec<&str> = (&text[index+4..index+4+a]).split(',').collect(); 

            let mut num = 1;
            if arr.len() == 2 {
                for ele in arr {
                    match ele.parse::<i32>() {
                        Ok(val) => num *= val,
                        Err(_) => {
                            index += 4;
                            continue 'out;
                        }
                    }
                }
                val1 += num;
                if con {
                    val2 += num;
                }
                index += 3+a;
            }
        }
        index += 1;
    }
    println!("Problem 1: {}", val1);
    println!("Problem 2: {}", val2);
}
