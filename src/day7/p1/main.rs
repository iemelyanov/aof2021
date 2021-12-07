use std::io::{BufRead, BufReader};

fn f(input: &mut Vec<usize>) -> i64 {
    let avg = (input.iter().sum::<usize>() as f64 / input.len() as f64).round() as i64;
    let mut min = i64::MAX;
    for i in (0..=avg).rev() {
        let t = input.iter().map(|v| (*v as i64 - i).abs()).sum::<i64>();
        if t < min {
            min = t
        } else {
            break;
        }
    }

    min
}

fn main() {
    let mut buf = String::new();
    BufReader::new(std::io::stdin())
        .read_line(&mut buf)
        .unwrap();
    let mut input: Vec<usize> = buf.split(",").map(|s| s.parse().unwrap()).collect();
    println!("{}", f(&mut input));
}
