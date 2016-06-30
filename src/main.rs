use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
// use std::collections::HashMap;

trait TupleReader {
    fn next_tuple(&mut self) -> (u32, u32);
}

impl<T: BufRead> TupleReader for T {
    fn next_tuple(&mut self) -> (u32, u32) {
        let mut buffer = String::new();
        self.read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace().map(|s| s.parse().unwrap()).take(2);
        (iter.next().unwrap(), iter.next().unwrap())
    }
}

#[derive(Debug)]
struct Graph {
    edges: Vec<(u32, u32)>
}

impl Graph {
    fn load<T: TupleReader>(reader: &mut T) -> Graph {
        let (_, e) = reader.next_tuple();
        let mut edges = vec![];
        for _ in 0..e { 
            let edge = reader.next_tuple();
            edges.push(edge)
        }
        Graph { edges: edges }
    }

}

fn main() {
    let filename = "graph.txt";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(&file);
    let graph = Graph::load(&mut reader);
    println!("{:?}", graph);
    let (from, to) = reader.next_tuple();
    println!("Checking reachability {} -> {}", from, to);
}
