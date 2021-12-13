use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stdin, BufRead, BufReader};
use std::rc::Rc;

struct Graph {
    graph: HashMap<String, Rc<RefCell<Vec<String>>>>,
    visited: HashMap<String, usize>,
    c: usize,
}

impl Graph {
    fn new() -> Self {
        Self {
            graph: HashMap::new(),
            visited: HashMap::new(),
            c: 0,
        }
    }

    fn add_edge(&mut self, u: String, v: String) {
        self.graph
            .entry(u.clone())
            .or_insert(Rc::new(RefCell::new(Vec::new())))
            .borrow_mut()
            .push(v.clone());
        self.graph
            .entry(v)
            .or_insert(Rc::new(RefCell::new(Vec::new())))
            .borrow_mut()
            .push(u);
    }

    fn count_all_paths(&mut self, start: &str, end: &str) -> usize {
        self.c += 1;
        if !start.chars().any(|c| c.is_uppercase()) {
            *self.visited.entry(start.to_string()).or_insert(0) += 1;
        }

        let mut cnt = 0;
        if start == end {
            cnt += 1;
        } else if let Some(vertices) = self.graph.get(start) {
            for v in Rc::clone(vertices).borrow().iter() {
                let mut max = 2;
                if v == "start" || v == "end" || self.visited.iter().any(|(_, &qty)| qty >= 2) {
                    max = 1;
                }
                if *self.visited.get(v).unwrap_or(&0) >= max {
                    continue;
                }
                cnt += self.count_all_paths(v, end);
            }
        }
        self.visited.get_mut(start).map(|v| {
            if *v > 0 {
                *v -= 1;
            }
        });

        cnt
    }
}

fn main() {
    let inputs = BufReader::new(stdin()).lines().map(|l| l.unwrap());
    let mut g = Graph::new();
    for i in inputs {
        let s: Vec<&str> = i.split("-").collect();
        g.add_edge(s[0].to_string(), s[1].to_string());
    }
    println!("{:?}", g.count_all_paths("start", "end"));
}
