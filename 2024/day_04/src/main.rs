use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut text = String::new();
    reader.read_to_string(&mut text);
    let mat: Vec<Vec<char>> = text.trim().split('\n').map( |x| x.chars().collect()).collect();
    
    let mut val1 = 0;
    let mut val2 = 0;
    for row in 0..mat.len() {
        for column in 0..mat[row].len() {
            // Problem 1
            if mat[row][column] == 'X' {
                let left = column > 2;
                let right = column <= mat[row].len() - 4;
                let top = row > 2;
                let bottom = row <= mat.len() - 4;
                
                if left {
                    if mat[row][column-3..column+1].iter().collect::<String>() == "SAMX" {
                        val1 += 1;
                    }

                    if top {
                        let mut a = String::with_capacity(4);
                        for x in 0..4 {
                            a.push_str(mat[row-x][column-x].to_string().as_str());
                        }
                        if a == "XMAS" {
                            val1 += 1;
                        }
                    }
                    if bottom {
                        let mut a = String::with_capacity(4);
                        for x in 0..4 {
                            a.push_str(mat[row+x][column-x].to_string().as_str());
                        }
                        if a == "XMAS" {
                            val1 += 1;
                        }
                    }
                }

                if right {
                    if mat[row][column..column+4].iter().collect::<String>() == "XMAS" {
                        val1 += 1;
                    }

                    if top {
                        let mut a = String::with_capacity(4);
                        for x in 0..4 {
                            a.push_str(mat[row-x][column+x].to_string().as_str());
                        }
                        if a == "XMAS" {
                            val1 += 1;
                        }

                    }
                    if bottom {
                        let mut a = String::with_capacity(4);
                        for x in 0..4 {
                            a.push_str(mat[row+x][column+x].to_string().as_str());
                        }
                        if a == "XMAS" {
                            val1 += 1;
                        }

                    }
                }

                if top {
                        let mut a = String::with_capacity(4);
                        for x in 0..4 {
                            a.push_str(mat[row-x][column].to_string().as_str());
                        }
                        if a == "XMAS" {
                            val1 += 1;
                        }

                }

                if bottom {
                        let mut a = String::with_capacity(4);
                        for x in 0..4 {
                            a.push_str(mat[row+x][column].to_string().as_str());
                        }
                        if a == "XMAS" {
                            val1 += 1;
                        }

                }
            }
            
            // Problem 2
            if mat[row][column] == 'A' {
                if row > 0 && row < mat.len() - 1 && column > 0 && column < mat[row].len() - 1 {
                    let s1 = format!("{}{}{}", mat[row-1][column-1], mat[row][column], mat[row+1][column+1]);
                    let s2 = format!("{}{}{}", mat[row+1][column-1], mat[row][column], mat[row-1][column+1]);
                    if (s1 == "MAS" || s1 == "SAM") && (s2 == "MAS" || s2 == "SAM") {
                        val2 += 1;
                    }
                }
            }   
        }
    }
    println!("Problem 1: {val1}");
    println!("Problem 2: {val2}");
}
