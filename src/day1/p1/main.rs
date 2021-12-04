use std::io::{BufRead, BufReader};

fn main() {
    let mut prev = 0;
    let mut cnt = 0;
    let input = BufReader::new(std::io::stdin());
    for line in input.lines() {
        let cur = line.unwrap().parse::<i32>().unwrap();
        cnt += (cur > prev) as i32;
        prev = cur;
    }
    println!("{}", cnt - 1);
}
