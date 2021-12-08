use std::io::{BufRead, BufReader};

fn match_chars(source: &str, target: &str) -> bool {
    for t in source.chars() {
        if !target.contains(t) {
            return false;
        }
    }
    true
}

fn main() {
    let reader = BufReader::new(std::io::stdin()).lines().map(|l| l.unwrap());
    let mut sum = 0;
    for line in reader {
        let t: Vec<&str> = line.split("|").collect();
        let pattern: Vec<&str> = t[0].trim().split(" ").collect();
        let mut p1: Option<&str> = None;
        let mut p4: Option<&str> = None;
        for p in pattern {
            match p.len() {
                2 => p1 = Some(p),
                4 => p4 = Some(p),
                _ => {}
            };
        }
        let pl: String = p4
            .unwrap()
            .chars()
            .filter(|c| !p1.unwrap().contains(*c))
            .collect();

        let out_sig: Vec<&str> = t[1].trim().split(" ").collect();
        let mut n = 0;
        let mut k = 1;
        for s in out_sig.iter().rev() {
            n += k * match s.len() {
                2 => 1,
                4 => 4,
                3 => 7,
                7 => 8,
                5 => {
                    if match_chars(p1.unwrap(), s) {
                        3
                    } else if match_chars(&pl, s) {
                        5
                    } else {
                        2
                    }
                }
                6 => {
                    if !match_chars(p1.unwrap(), s) {
                        6
                    } else if match_chars(p4.unwrap(), s) {
                        9
                    } else {
                        0
                    }
                }
                _ => 0,
            };
            k *= 10;
        }
        sum += n;
    }
    println!("{}", sum);
}
