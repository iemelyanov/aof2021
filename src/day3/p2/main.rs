use std::collections::HashSet;
use std::io::{BufRead, BufReader};

const N: i32 = 12;

fn f(numbers: &HashSet<i32>, i: i32, inverse: bool) -> i32 {
    if numbers.len() == 1 {
        return *numbers.into_iter().next().unwrap();
    }

    let cnt: usize = numbers.iter().filter(|n| *n & (1 << i) > 0).count();
    let z = (cnt >= (numbers.len() - cnt)) ^ inverse;
    let numbers: HashSet<i32> = numbers
        .iter()
        .filter(|n| (*n & (1 << i) > 0) == z)
        .map(|n| *n)
        .collect();

    f(&numbers, i - 1, inverse)
}

fn main() {
    let input = BufReader::new(std::io::stdin());
    let mut numbers: HashSet<i32> = HashSet::new();
    for line in input.lines() {
        let mut x = 0;
        for c in line.unwrap().chars().into_iter() {
            if c == '1' {
                x |= 1;
            }
            x <<= 1;
        }
        numbers.insert(x >> 1);
    }
    let oxygen_generator_rating = f(&numbers, N - 1, false);
    let co2_scrubber_rating = f(&numbers, N - 1, true);
    println!("{}", oxygen_generator_rating * co2_scrubber_rating);
}
