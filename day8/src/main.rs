use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type BoxPair = (i64, ((i64, i64, i64), (i64, i64, i64)));

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        let mut parent = vec![0; size];
        for i in 0..size {
            parent[i] = i;
        };
        let size = vec![1; size];
        
        DisjointSet { parent, size }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        }
        else {
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_i = self.find(a);
        let root_j = self.find(b);
        
        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
        }
    }
}

fn part_one(junc_boxes: &[(i64, i64, i64)]) -> i64 {
    let mut mapped_boxes: HashMap<(i64, i64, i64), i64> = HashMap::new();

    for (i, junc_box) in junc_boxes.iter().enumerate() {
        mapped_boxes.insert(*junc_box, i as i64);
    }

    let mut distances: Vec<BoxPair> = Vec::new();

    for (i, junc_box) in junc_boxes.iter().enumerate() {
        for j in i+1..junc_boxes.len() {
            let distance = calculate_distance(junc_box, &junc_boxes[j]);
            distances.push((distance, ((*junc_box), (junc_boxes[j]))));
        }
    }

    distances.sort_by(|a, b| a.0.cmp(&b.0));

    let top_1000 = &distances[0..10];
    
    let mut union_graph = DisjointSet::new(junc_boxes.len());

    for boxes in top_1000 {
        let id_a = *mapped_boxes.get(&boxes.1.0).unwrap() as usize;
        let id_b = *mapped_boxes.get(&boxes.1.1).unwrap() as usize;

        if union_graph.find(id_a) != union_graph.find(id_b) {
            union_graph.union(id_a, id_b);
        }
    }

    union_graph.size.sort_by(|a, b| a.cmp(b));

    let sum: i64 = union_graph.size[union_graph.size.len() - 3..].iter().fold(1, |acc, x| acc * x) as i64;

    return sum;
}

fn calculate_distance(first_box: &(i64, i64, i64), second_box: &(i64, i64, i64)) -> i64 {
    let distance = ((second_box.0 - first_box.0).pow(2) + 
                        (second_box.1 - first_box.1).pow(2) + 
                        (second_box.2 - first_box.2).pow(2)).isqrt();
    return distance;
}

fn main() {
    let path = "/home/alejoseed/Projects/aoc2025/day8/src/1.input";
    let lines_result = read_lines(path);
    
    let lines = match lines_result {
        Ok(l) => l.map_while(Result::ok),
        Err(e) => {
            eprintln!("Error reading file at {}: {}", path, e);
            return;
        }
    };

    let mut junc_boxes: Vec<(i64, i64, i64)> = Vec::new();

    for line in lines {
        let coordinate: Vec<&str> = line.split(",").collect();

        let x = coordinate.get(0).unwrap().parse::<i64>().unwrap();
        let y = coordinate.get(1).unwrap().parse::<i64>().unwrap();
        let z = coordinate.get(2).unwrap().parse::<i64>().unwrap();

        junc_boxes.push((x,y,z));
    }

    let first_result = part_one(&junc_boxes);

    println!("{}", first_result);
    // println!("STOP HERE");
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P : AsRef<Path> {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines()); 
}