use std::io::{BufRead, BufReader};

fn is_min_height(hm: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    if i + 1 < hm.len() {
        if hm[i + 1][j] <= hm[i][j] {
            return false;
        }
    }
    if i > 0 {
        if hm[i - 1][j] <= hm[i][j] {
            return false;
        }
    }
    if j + 1 < hm.len() {
        if hm[i][j + 1] <= hm[i][j] {
            return false;
        }
    }
    if j > 0 {
        if hm[i][j - 1] <= hm[i][j] {
            return false;
        }
    }

    true
}

fn main() {
    let reader = BufReader::new(std::io::stdin()).lines().map(|l| l.unwrap());
    let mut height_map: Vec<Vec<u32>> = Vec::new();
    for line in reader {
        height_map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut risk_level = 0;
    for i in 0..height_map.len() {
        for j in 0..height_map[i].len() {
            if is_min_height(&height_map, i, j) {
                risk_level += height_map[i][j] + 1;
            }
        }
    }
    println!("{}", risk_level);
}
