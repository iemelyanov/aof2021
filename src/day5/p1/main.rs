use std::io::{BufRead, BufReader};

fn main() {
    let input = BufReader::new(std::io::stdin());
    let mut max_x = 0;
    let mut max_y = 0;
    let lines: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .map(|l| {
            let v: Vec<i32> = l
                .unwrap()
                .replace(" -> ", ",")
                .split(",")
                .map(|v| v.parse().unwrap())
                .collect();
            max_x = max_x.max(v[0]);
            max_x = max_x.max(v[2]);
            max_y = max_y.max(v[1]);
            max_y = max_y.max(v[3]);
            (v[0], v[1], v[2], v[3])
        })
        .collect();

    let mut map = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    for (x1, y1, x2, y2) in lines {
        if !(x1 == x2 || y1 == y2) {
            continue;
        }

        if x1 == x2 {
            let (from_y, to_y) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
            let x = x1;
            for y in from_y..=to_y {
                map[y as usize][x as usize] += 1;
            }
            continue;
        }

        if y1 == y2 {
            let (from_x, to_x) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
            let y = y1;
            for x in from_x..=to_x {
                map[y as usize][x as usize] += 1;
            }
            continue;
        }
    }

    let mut at_least_two_cnt = 0;
    for v in map {
        for q in v {
            at_least_two_cnt += (q > 1) as i32;
        }
    }

    println!("{}", at_least_two_cnt);
}
