use std::io::{BufRead, BufReader};

const N: usize = 12;

fn main() {
    let input = BufReader::new(std::io::stdin());
    let mut _1_cnts = [0; N];
    let mut _0_cnts = [0; N];
    for line in input.lines() {
        for (i, c) in line.unwrap().chars().into_iter().enumerate() {
            if c == '1' {
                _1_cnts[i] += 1;
            } else {
                _0_cnts[i] += 1;
            }
        }
    }
    let mut gamma_rate = 0;
    for i in 0..N - 1 {
        if _1_cnts[i] > _0_cnts[i] {
            gamma_rate |= 1;
        }
        gamma_rate <<= 1;
    }
    let epsilon_rate = gamma_rate ^ 0xfff;
    println!("{}", gamma_rate * epsilon_rate);
}
