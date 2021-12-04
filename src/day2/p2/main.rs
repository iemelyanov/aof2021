use std::io::{BufRead, BufReader};

fn main() {
    let input = BufReader::new(std::io::stdin());
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (a, b): (&str, &str) = line.split_at(line.find(" ").unwrap());
        let cmd = a;
        let val: i32 = b.trim().parse().unwrap();
        match cmd {
            "forward" => {
                h += val;
                d += aim * val;
            }
            "down" => aim += val,
            "up" => aim -= val,
            _ => {}
        }
    }
    println!("{}", d * h);
}
