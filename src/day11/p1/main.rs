use std::io::{BufRead, BufReader};

struct Grid {
    grid: Vec<Vec<i32>>,
    visited: Vec<Vec<bool>>,
    w: usize,
    h: usize,
}

impl Grid {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let h = grid.len();
        let w = if grid.len() > 0 { grid[0].len() } else { 0 };
        Self {
            grid: grid,
            visited: vec![vec![false; w]; h],
            w,
            h,
        }
    }

    fn valid(&self, i: i32, j: i32) -> bool {
        i >= 0 && i < self.w as i32 && j >= 0 && j < self.h as i32
    }

    fn all_flushed(&self) -> bool {
        for r in self.grid.iter() {
            for c in r.iter() {
                if *c != 0 {
                    return false;
                }
            }
        }
        true
    }

    fn incr_all(&mut self) {
        for i in 0..self.h {
            for j in 0..self.w {
                self.grid[i][j] += 1;
                self.visited[i][j] = false;
            }
        }
    }

    fn incr_and_try_flush(&mut self, i: i32, j: i32) -> i32 {
        if self.valid(i, j) && !self.visited[i as usize][j as usize] {
            self.grid[i as usize][j as usize] += 1;
            if self.grid[i as usize][j as usize] > 9 {
                return self.try_flash(i, j);
            }
        }

        0
    }

    fn try_flash(&mut self, i: i32, j: i32) -> i32 {
        if !self.valid(i, j) {
            return 0;
        }
        if self.visited[i as usize][j as usize] || self.grid[i as usize][j as usize] <= 9 {
            return 0;
        }

        self.grid[i as usize][j as usize] = 0;
        self.visited[i as usize][j as usize] = true;

        self.incr_and_try_flush(i - 1, j - 1)
            + self.incr_and_try_flush(i - 1, j)
            + self.incr_and_try_flush(i - 1, j + 1)
            + self.incr_and_try_flush(i, j - 1)
            + self.incr_and_try_flush(i, j + 1)
            + self.incr_and_try_flush(i + 1, j - 1)
            + self.incr_and_try_flush(i + 1, j)
            + self.incr_and_try_flush(i + 1, j + 1)
            + 1
    }
}

fn main() {
    let lines = BufReader::new(std::io::stdin()).lines().map(|l| l.unwrap());
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for l in lines {
        grid.push(l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect());
    }

    {
        // Part 1
        let mut grid = Grid::new(grid.clone());
        let mut cnt = 0;
        for _ in 0..100 {
            grid.incr_all();
            for i in 0..grid.h {
                for j in 0..grid.w {
                    cnt += grid.try_flash(i as i32, j as i32);
                }
            }
        }
        println!("\n{} flashes", cnt);
    }
    {
        // Part 2
        let mut grid = Grid::new(grid);
        let mut i = 0;
        while !grid.all_flushed() {
            grid.incr_all();
            for i in 0..grid.h {
                for j in 0..grid.w {
                    grid.try_flash(i as i32, j as i32);
                }
            }
            i += 1;
        }
        println!("first step during which all octopuses flash: {} ", i);
    }
}
