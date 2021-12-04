use std::io::{BufRead, BufReader};

fn f(board: &mut [[(i32, bool); 6]; 6], n: i32) -> i32 {
    for i in 0..5 {
        for j in 0..5 {
            if n == board[i][j].0 && !board[i][j].1 {
                board[i][j].1 = true;
                board[i][5].0 += 1;
                board[5][j].0 += 1;
                if board[i][5].0 == 5 || board[5][j].0 == 5 {
                    return n;
                }
            }
        }
    }

    0
}

fn main() {
    let input = BufReader::new(std::io::stdin());
    let mut lines = input.lines();
    let numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    let mut boards = vec![[[(0, false); 6]; 6]];
    boards.push([[(0, false); 6]; 6]);
    let mut i = 0;
    for line in lines.map(|l| l.unwrap()) {
        if line.len() == 0 {
            i = 0;
            boards.push([[(0, false); 6]; 6]);
            continue;
        }
        for (j, n) in line
            .split_ascii_whitespace()
            .map(|v| v.trim().parse::<i32>().unwrap())
            .enumerate()
        {
            boards.last_mut().unwrap()[i][j].0 = n;
        }
        i += 1;
    }

    let mut last_winner_idx = 0;
    let mut last_winner_n = 0;
    let mut winners_idx = vec![false; boards.len()];
    for n in numbers {
        for (i, b) in boards.iter_mut().enumerate() {
            if winners_idx[i] {
                continue;
            }
            let n = f(b, n);
            if n != 0 {
                winners_idx[i] = true;
                last_winner_idx = i;
                last_winner_n = n;
            }
        }
    }

    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !boards[last_winner_idx][i][j].1 {
                sum += boards[last_winner_idx][i][j].0;
            }
        }
    }
    println!("{}", sum * last_winner_n);
    return;
}
