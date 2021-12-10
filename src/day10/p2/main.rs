use std::collections::LinkedList;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = BufReader::new(std::io::stdin()).lines().map(|l| l.unwrap());
    let mut scores = Vec::new();
    for line in lines {
        let mut chunks = LinkedList::new();
        let mut corrupted = false;
        for c in line.chars() {
            match c {
                ')' => {
                    if let Some(v) = chunks.back() {
                        if *v != '(' {
                            corrupted = true;
                            break;
                        }
                        chunks.pop_back();
                    }
                }
                ']' => {
                    if let Some(v) = chunks.back() {
                        if *v != '[' {
                            corrupted = true;
                            break;
                        }
                        chunks.pop_back();
                    }
                }
                '>' => {
                    if let Some(v) = chunks.back() {
                        if *v != '<' {
                            corrupted = true;
                            break;
                        }
                        chunks.pop_back();
                    }
                }
                '}' => {
                    if let Some(v) = chunks.back() {
                        if *v != '{' {
                            corrupted = true;
                            break;
                        }
                        chunks.pop_back();
                    }
                }
                _ => chunks.push_back(c),
            }
        }
        if !corrupted {
            let mut score = 0 as i64;
            for c in chunks.iter().rev() {
                score *= 5;
                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}
