use std::collections::LinkedList;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = BufReader::new(std::io::stdin()).lines().map(|l| l.unwrap());
    let mut points = 0;
    for line in lines {
        let mut chunks = LinkedList::new();
        for c in line.chars() {
            match c {
                ')' => {
                    if let Some(v) = chunks.back() {
                        if *v != '(' {
                            points += 3;
                            break;
                        }
                        chunks.pop_back();
                    }
                }
                ']' => {
                    if let Some(v) = chunks.back() {
                        if *v != '[' {
                            points += 57;
                            break;
                        }
                        chunks.pop_back();
                    }
                }
                '>' => {
                    if let Some(v) = chunks.back() {
                        if *v != '<' {
                            points += 25137;
                            break;
                        }
                        chunks.pop_back();
                    }
                }
                '}' => {
                    if let Some(v) = chunks.back() {
                        if *v != '{' {
                            points += 1197;
                            break;
                        }
                        chunks.pop_back();
                    }
                }
                _ => chunks.push_back(c),
            }
        }
    }
    println!("{}", points);
}
