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
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        let dim: Vec<usize> = buffer.split_whitespace().map(|s| s.parse().unwrap()).collect();
        GraphReader { buf: reader, vertices: dim[0], edges: dim[1] }
    }

    fn adjacencies(self) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();
        for line in self.buf.lines().take(self.edges).map(|s| s.unwrap()) {
            let edge: Vec<&str> = line.split_whitespace().collect();
            let vertex = edge[0].to_string();
            let neighbor = edge[1].to_string();
            if !graph.contains_key(&vertex) {
                graph.insert(vertex, vec![neighbor]);
            } else {
                graph.get_mut(&vertex).unwrap().push(neighbor);
            }
        }
        graph
    }
}

fn main() {
    let filename = "graph.txt";
    let file = File::open(filename).unwrap();
    let graph = GraphReader::new(BufReader::new(&file));
    let adj = graph.adjacencies();
    for (vertex, neighbors) in adj {
        println!("node {}:", vertex);
        for n in neighbors {
            println!("\t{} -> {}", vertex, n);
        }
    }
}
