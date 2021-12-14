use std::io::{stdin, BufRead, BufReader};

#[derive(Debug)]
enum FoldDirection {
    X(usize),
    Y(usize),
}

fn fold_space(space: Vec<Vec<char>>, fold_direction: FoldDirection) -> Vec<Vec<char>> {
    match fold_direction {
        FoldDirection::X(x) => {
            let mut a: Vec<Vec<char>> = Vec::new();
            let mut b: Vec<Vec<char>> = Vec::new();
            space.iter().for_each(|v| {
                let (l, r) = v.split_at(x);
                a.push(l.iter().map(|c| *c).collect::<Vec<char>>());
                b.push(r.iter().map(|c| *c).collect::<Vec<char>>());
            });
            let mut r: Vec<Vec<char>> = a.iter().map(|l| l.clone()).collect();
            for (i, v) in b.iter().enumerate() {
                for (j, c) in v.iter().enumerate() {
                    if *c == '#' {
                        r[i][x - j] = *c;
                    }
                }
            }
            r
        }
        FoldDirection::Y(y) => {
            let (a, b) = space.split_at(y);
            let mut r: Vec<Vec<char>> = a.iter().map(|l| l.clone()).collect();
            for (i, v) in b.iter().enumerate() {
                for (j, c) in v.iter().enumerate() {
                    if *c == '#' {
                        r[y - i][j] = *c;
                    }
                }
            }
            r
        }
    }
}

fn main() {
    let mut lines = BufReader::new(stdin()).lines().map(|l| l.unwrap());
    let mut dots: Vec<(usize, usize)> = Vec::new();
    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }
        let (a, b) = l.split_once(",").unwrap();
        dots.push((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
    }
    let mut fold_directions: Vec<FoldDirection> = Vec::new();
    while let Some(l) = lines.next() {
        let (a, b) = l.split_once("=").unwrap();
        let v = b.parse::<usize>().unwrap();
        let d = match a.as_bytes()[a.len() - 1] {
            b'x' => FoldDirection::X(v),
            b'y' => FoldDirection::Y(v),
            _ => unreachable!(),
        };
        fold_directions.push(d);
    }

    let width = dots.iter().max_by_key(|p| p.0).map(|p| p.0).unwrap();
    let height = dots.iter().max_by_key(|p| p.1).map(|p| p.1).unwrap();

    let mut space = vec![vec!['.'; width + 1]; height + 1];
    for (i, j) in dots {
        space[j][i] = '#'
    }

    for fd in fold_directions {
        space = fold_space(space, fd);
        println!();
        let mut cnt = 0;
        for c in space.iter() {
            cnt += c.iter().filter(|p| **p == '#').count();
        }
        println!("dots: {}", cnt);
    }
    println!();
    for c in space.iter() {
        println!("{:?}", c);
    }
}
