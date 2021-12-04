use std::io::{BufRead, BufReader};

fn main() {
    let input = BufReader::new(std::io::stdin());
    let mut h = 0;
    let mut d = 0;
    for line in input.lines() {
        let line = line.unwrap();
        let (a, b): (&str, &str) = line.split_at(line.find(" ").unwrap());
        let cmd = a;
        let val: i32 = b.trim().parse().unwrap();
        match cmd {
            "forward" => h += val,
            "down" => d += val,
            "up" => {
                let x = d - val;
                d = if x < 0 { 0 } else { x }
            }
            _ => {}
        }
    }
    println!("{}", d * h);
}
