use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    io::{stdin, BufRead, BufReader},
};

#[derive(Debug, Eq)]
struct Edge {
    w: i32,
    v: (i32, i32),
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.w.cmp(&other.w)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w
    }
}

fn dijkstra(map: &Vec<Vec<i32>>, from: (i32, i32), to: (i32, i32)) -> Option<i32> {
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let mut distance: HashMap<(i32, i32), (i32, bool)> = HashMap::new();
    for i in 0..height {
        for j in 0..width {
            distance.insert((i as i32, j as i32), (i32::MAX, false));
        }
    }
    distance.get_mut(&from).map(|v| v.0 = 0);

    let mut pq: BinaryHeap<Edge> = BinaryHeap::new();
    pq.push(Edge { w: 0, v: (0, 0) });
    while let Some(edge) = pq.pop() {
        let y = edge.v.0;
        let x = edge.v.1;
        let mut cur_distance = 0;
        if let Some((distance, visited)) = distance.get_mut(&(y, x)) {
            if *visited {
                continue;
            }
            *visited = true;
            cur_distance = *distance;
        }
        for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let y = y + i;
            let x = x + j;
            if let Some((distance, _)) = distance.get_mut(&(y, x)) {
                let w = map[y as usize][x as usize] + cur_distance;
                if w < *distance {
                    *distance = w;
                    pq.push(Edge {
                        w: -*distance,
                        v: (y, x),
                    });
                }
            }
        }
    }

    distance.get(&to).map(|v| v.0)
}

fn main() {
    let mut map: Vec<Vec<i32>> = Vec::new();
    let lines = BufReader::new(stdin()).lines();
    for l in lines.map(|l| l.unwrap()) {
        let r: Vec<i32> = l
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        map.push(r);
    }
    {
        // Part 1
        let height = map.len() as i32;
        let width = map[0].len() as i32;
        dijkstra(&map, (0, 0), (height - 1, width - 1))
            .map(|t| println!("(Part 1) Lowest total risk: {}", t));
    }
    {
        // Part 2
        let mut full_map: Vec<Vec<i32>> = Vec::new();
        for i in 0..5 {
            for r in map.iter() {
                let mut v: Vec<i32> = Vec::new();
                for j in 0..5 {
                    if i == 0 && j == 0 {
                        v.append(&mut r.clone());
                    } else {
                        let mut tmp: Vec<i32> = r
                            .iter()
                            .map(|n| {
                                let x = n + j + i;
                                if x > 9 {
                                    x % 10 + 1
                                } else {
                                    x
                                }
                            })
                            .collect();
                        v.append(&mut tmp);
                    }
                }
                full_map.push(v);
            }
        }

        let height = full_map.len() as i32;
        let width = full_map[0].len() as i32;
        dijkstra(&full_map, (0, 0), (height - 1, width - 1))
            .map(|t| println!("(Part 2) Lowest total risk: {}", t));
    }
}
