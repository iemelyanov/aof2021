use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(std::io::stdin()).lines().map(|l| l.unwrap());
    let mut cnt = 0;
    for line in reader {
        let t: Vec<&str> = line.split("|").collect();
        let out_sig: Vec<&str> = t[1].trim().split(" ").collect();
        for s in out_sig {
            cnt += match s.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            };
        }
    }
    println!("{}", cnt);
}
