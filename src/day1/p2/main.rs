use std::io::{BufRead, BufReader};

const N: usize = 3;

fn main() {
    let mut prev = 0;
    let mut cnt = 0;
    let mut window = [0; N];
    let input = BufReader::new(std::io::stdin());
    for (i, line) in input.lines().enumerate() {
        window[i % N] = line.unwrap().parse().unwrap();
        let mut cur = 0;
        for j in 0..N {
            cur += window[j];
        }
        cnt += (cur > prev) as usize;
        prev = cur;
    }
    println!("{}", cnt - N);
}
