use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

struct GraphReader<T: BufRead> {
    buf: T,
    vertices: usize,
    edges: usize,
}

impl<T: BufRead> GraphReader<T> {
    fn new(mut reader: T) -> GraphReader<T> {
        let dim = GraphReader::next_tuple(&mut reader);
        let v = dim.0.parse().unwrap();
        let e = dim.1.parse().unwrap();
        GraphReader { buf: reader, vertices: v, edges: e }
    }

    fn next_tuple(buf: &mut T) -> (String, String) {
        let mut buffer = String::new();
        buf.read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        (iter.next().unwrap().to_string(), iter.next().unwrap().to_string())
    }

    fn adjacencies(&mut self) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();
        for _ in 0..self.edges { 
            let vertices = GraphReader::next_tuple(&mut self.buf);
            let a = vertices.0.to_string();
            let b = vertices.1.to_string();
            if !graph.contains_key(&a) {
                graph.insert(a, vec![b]);
            } else {
                graph.get_mut(&a).unwrap().push(b);
            }
        }
        graph
    }
}

fn main() {
    let filename = "graph.txt";
    let file = File::open(filename).unwrap();
    let mut graph = GraphReader::new(BufReader::new(&file));
    let adj = graph.adjacencies();
    for (vertex, neighbors) in adj {
        println!("node {}:", vertex);
        for n in neighbors {
            println!("\t{} -> {}", vertex, n);
        }
    }
}
