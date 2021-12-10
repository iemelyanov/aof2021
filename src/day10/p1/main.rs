use std::io::{BufRead, BufReader};

fn main() {
    let lines = BufReader::new(std::io::stdin()).lines().map(|l| l.unwrap());
    let mut points = 0;
    for line in lines {
        let mut chunks = Vec::with_capacity(line.len());
        for c in line.chars() {
            match c {
                ')' => {
                    if let Some(v) = chunks.last() {
                        if *v != '(' {
                            points += 3;
                            break;
                        }
                        chunks.pop();
                    }
                }
                ']' => {
                    if let Some(v) = chunks.last() {
                        if *v != '[' {
                            points += 57;
                            break;
                        }
                        chunks.pop();
                    }
                }
                '>' => {
                    if let Some(v) = chunks.last() {
                        if *v != '<' {
                            points += 25137;
                            break;
                        }
                        chunks.pop();
                    }
                }
                '}' => {
                    if let Some(v) = chunks.last() {
                        if *v != '{' {
                            points += 1197;
                            break;
                        }
                        chunks.pop();
                    }
                }
                _ => chunks.push(c),
            }
        }
    }
    println!("{}", points);
}
