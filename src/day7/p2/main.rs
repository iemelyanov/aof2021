use std::io::{BufRead, BufReader};

fn f(input: &mut Vec<usize>) -> usize {
    let max = *input.iter().max().unwrap();
    let mut d = vec![0; max + 1];
    d[0] = 0;
    d[1] = 1;
    for i in 2..=max {
        d[i] = i + d[i - 1];
    }

    let avg = (input.iter().sum::<usize>() as f64 / input.len() as f64).round() as i64;
    let mut min = usize::MAX;
    for i in (0..=avg).rev() {
        let t: usize = input
            .iter()
            .map(|v| d[(*v as i64 - i).abs() as usize])
            .sum();
        if t < min {
            min = t;
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
