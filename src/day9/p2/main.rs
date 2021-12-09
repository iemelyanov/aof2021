use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader};

fn basin_size(hm: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    if !(i < hm.len() as i32 && i >= 0 && j < hm[0].len() as i32 && j >= 0) {
        return 0;
    }

    if hm[i as usize][j as usize] == 9 || hm[i as usize][j as usize] == -1 {
        return 0;
    }
    hm[i as usize][j as usize] = -1;

    1 + basin_size(hm, i + 1, j)
        + basin_size(hm, i - 1, j)
        + basin_size(hm, i, j + 1)
        + basin_size(hm, i, j - 1)
}

fn main() {
    let reader = BufReader::new(std::io::stdin()).lines().map(|l| l.unwrap());
    let mut height_map: Vec<Vec<i32>> = Vec::new();
    for line in reader {
        height_map.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }

    let mut sizes = BinaryHeap::new();
    for i in 0..height_map.len() {
        for j in 0..height_map[i].len() {
            let size = basin_size(&mut height_map, i as i32, j as i32);
            if size > 0 {
                sizes.push(size);
            }
        }
    }

    println!(
        "{}",
        sizes.pop().unwrap_or(0) * sizes.pop().unwrap_or(0) * sizes.pop().unwrap_or(0)
    );
}
