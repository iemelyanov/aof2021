use std::io::{BufRead, BufReader};

fn match_pair(open: char, close: char) -> bool {
    match open {
        '(' => close == ')',
        '[' => close == ']',
        '{' => close == '}',
        '<' => close == '>',
        _ => unreachable!(),
    }
}

fn main() {
    let lines = BufReader::new(std::io::stdin()).lines().map(|l| l.unwrap());
    let mut scores = Vec::new();
    for line in lines {
        let mut chunks: Vec<char> = Vec::with_capacity(line.len());
        let mut corrupted = false;
        for c in line.chars() {
            match c {
                ')' | ']' | '>' | '}' => {
                    if let Some(o) = chunks.last() {
                        if !match_pair(*o, c) {
                            corrupted = true;
                            break;
                        }
                    }
                    chunks.pop();
                }
                _ => chunks.push(c),
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
                    _ => unreachable!(),
                };
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}
