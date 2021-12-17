use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut reader = BufReader::new(stdin());
    let mut template = String::new();
    reader.read_line(&mut template).unwrap();
    println!("{}", template);

    let lines = reader.lines();
    let mut rules = HashMap::new();
    for line in lines.map(|l| l.unwrap()).filter(|l| l.len() > 0) {
        line.split_once(" -> ").map(|e| {
            let key = [e.0.as_bytes()[0], e.0.as_bytes()[1]];
            let val = e.1.as_bytes()[0];
            rules.insert(key, val);
        });
    }
    let mut m = HashMap::new();
    for i in 2..template.len() {
        let k = &template[i - 2..i];
        *m.entry([k.as_bytes()[0], k.as_bytes()[1]]).or_insert(0) += 1;
    }
    for i in 1..=40 {
        let mut t = HashMap::new();
        for (k, n) in m.iter() {
            if let Some(c) = rules.get(k) {
                let key1 = [k[0], *c];
                let key2 = [*c, k[1]];
                *t.entry(key1).or_insert(0) += *n;
                *t.entry(key2).or_insert(0) += *n;
            } else {
                t.insert(k.clone(), *n);
            }
        }
        m = t;
        let mut cnt: [u64; 26] = [0; 26];
        for (k, n) in m.iter() {
            cnt[(k[0] - b'A') as usize] += n;
        }
        let max = cnt.iter().max_by_key(|k| **k).unwrap();
        let min = cnt.iter().filter(|e| **e > 0).min_by_key(|k| **k).unwrap();
        println!("After step {}:\t{}", i, max - min + 1);
    }
}
