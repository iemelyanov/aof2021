use std::io::{BufRead, BufReader};

fn main() {
    let input: Vec<String> = BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let input: Vec<usize> = input[0].split(",").map(|s| s.parse().unwrap()).collect();
    let mut counters = vec![0; 9];
    for i in input {
        counters[i] += 1;
    }
    for _ in 1..=80 {
        let t = counters[0];
        let mut j = 0;
        for i in 1..=8 {
            counters[j] = counters[i];
            counters[i] = 0;
            j += 1;
        }
        if t > 0 {
            counters[6] += t;
            counters[8] = t;
        }
    }
    println!("{}", counters.iter().sum::<i32>());
}
