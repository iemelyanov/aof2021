use std::collections::HashSet;
use std::io::{stdin, BufRead, BufReader};

#[derive(Debug)]
enum FoldDirection {
    X(usize),
    Y(usize),
}

fn fold_space(
    space: HashSet<(usize, usize)>,
    fold_direction: FoldDirection,
) -> HashSet<(usize, usize)> {
    let ita = space.iter().map(|p| *p);
    let itb = space.iter();
    match fold_direction {
        FoldDirection::X(x) => ita
            .filter(|p| p.0 <= x)
            .collect::<HashSet<_>>()
            .union(
                &itb.filter(|p| p.0 > x)
                    .map(|p| (2 * x - p.0, p.1))
                    .collect(),
            )
            .map(|p| *p)
            .collect(),
        FoldDirection::Y(y) => ita
            .filter(|p| p.1 <= y)
            .collect::<HashSet<_>>()
            .union(
                &itb.filter(|p| p.1 > y)
                    .map(|p| (p.0, 2 * y - p.1))
                    .collect(),
            )
            .map(|p| *p)
            .collect(),
    }
}

fn main() {
    let mut lines = BufReader::new(stdin()).lines().map(|l| l.unwrap());
    let mut space: HashSet<(usize, usize)> = HashSet::new();
    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }
        let (a, b) = l.split_once(",").unwrap();
        space.insert((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
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

    for f in fold_directions {
        space = fold_space(space, f);
        println!("dots: {}", space.len());
    }

    // Draw
    println!();
    let width = space.iter().max_by_key(|p| p.0).unwrap().0;
    let height = space.iter().max_by_key(|p| p.1).unwrap().1;
    let mut v = vec![vec![' '; width + 1]; height + 1];
    for (i, j) in space.iter() {
        v[*j][*i] = '#';
    }
    for r in v {
        for c in r {
            print!("{}", c);
        }
        println!();
    }
}
