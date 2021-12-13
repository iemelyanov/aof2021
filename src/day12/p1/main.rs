use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead, BufReader};
use std::rc::Rc;

struct Graph {
    graph: HashMap<String, Rc<RefCell<Vec<String>>>>,
    visited: HashSet<String>,
    c: usize,
}

impl Graph {
    fn new() -> Self {
        Self {
            graph: HashMap::new(),
            visited: HashSet::new(),
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
            self.visited.insert(start.to_string());
        }

        let mut cnt = 0;
        if start == end {
            cnt += 1;
        } else if let Some(vertices) = self.graph.get(start) {
            for v in Rc::clone(vertices).borrow().iter() {
                if self.visited.contains(v) {
                    continue;
                }
                cnt += self.count_all_paths(v, end);
            }
        }
        self.visited.remove(start);

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
