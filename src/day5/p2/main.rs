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
            max_x = max_x.max(v[0]).max(v[2]);
            max_y = max_y.max(v[1]).max(v[3]);
            (v[0], v[1], v[2], v[3])
        })
        .collect();

    let mut map = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    for (mut x1, mut y1, x2, y2) in lines {
        // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = (y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = (if dx > dy { dx } else { -dy }) / 2;

        loop {
            map[y1 as usize][x1 as usize] += 1;
            if x1 == x2 && y1 == y2 {
                break;
            }
            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x1 += sx;
            }
            if e2 < dy {
                err += dx;
                y1 += sy;
            }
        }
    }

    let mut at_least_two_cnt = 0;
    for v in map {
        for q in v {
            // if q == 0 {
            //     print!(".");
            // } else {
            //     print!("{}", q);
            // }
            at_least_two_cnt += (q > 1) as i32;
        }
        // println!();
    }

    println!("{}", at_least_two_cnt);
}
