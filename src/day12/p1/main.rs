use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead, BufReader};
use std::rc::Rc;

struct Graph<'a> {
    graph: HashMap<&'a str, Rc<RefCell<Vec<&'a str>>>>,
    visited: HashSet<&'a str>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Self {
            graph: HashMap::new(),
            visited: HashSet::new(),
        }
    }

    fn add_edge(&mut self, u: &'a str, v: &'a str) {
        self.graph
            .entry(u)
            .or_insert(Rc::new(RefCell::new(Vec::new())))
            .borrow_mut()
            .push(v.clone());
        self.graph
            .entry(v)
            .or_insert(Rc::new(RefCell::new(Vec::new())))
            .borrow_mut()
            .push(u);
    }

    fn count_all_paths(&mut self, start: &'a str, end: &'a str) -> usize {
        if !start.chars().any(|c| c.is_uppercase()) {
            self.visited.insert(start);
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
