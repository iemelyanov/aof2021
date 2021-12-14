use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stdin, BufRead, BufReader};
use std::rc::Rc;

struct Graph<'a> {
    graph: HashMap<&'a str, Rc<RefCell<Vec<&'a str>>>>,
    visited: HashMap<&'a str, usize>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Self {
            graph: HashMap::new(),
            visited: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: &'a str, v: &'a str) {
        self.graph
            .entry(u)
            .or_insert(Rc::new(RefCell::new(Vec::new())))
            .borrow_mut()
            .push(v);
        self.graph
            .entry(v)
            .or_insert(Rc::new(RefCell::new(Vec::new())))
            .borrow_mut()
            .push(u);
    }

    fn count_all_paths(&mut self, start: &'a str, end: &'a str) -> usize {
        if !start.chars().any(|c| c.is_uppercase()) {
            *self.visited.entry(start).or_insert(0) += 1;
        }

        let mut cnt = 0;
        if start == end {
            cnt += 1;
        } else if let Some(vertices) = self.graph.get(start) {
            for v in Rc::clone(vertices).borrow().iter() {
                let mut max = 2;
                if *v == "start" || *v == "end" || self.visited.iter().any(|(_, &qty)| qty >= 2) {
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
    let inputs: Vec<String> = BufReader::new(stdin())
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let mut g = Graph::new();
    for i in inputs.iter() {
        let (u, v) = i.split_once("-").unwrap();
        g.add_edge(&u, &v);
    }
    println!("{:?}", g.count_all_paths("start", "end"));
}
