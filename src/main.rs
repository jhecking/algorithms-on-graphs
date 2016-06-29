use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Vertex {
    name: String,
}

impl Vertex {
    fn new(name: &str) -> Vertex {
        Vertex { name: name.to_string() }
    }
}

struct Edge {
    a: Vertex,
    b: Vertex,
}

impl Edge {
    fn new(a: &str, b: &str) -> Edge {
        Edge { a: Vertex::new(a), b: Vertex::new(b) }
    }
}

fn main() {
    let filename = "graph.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);
    let graph = load_graph(reader);
    for (v,es) in graph {
        println!("node {}:", v.name);
        for e in es {
            println!("\t{} -> {}", e.a.name, e.b.name);
        }
    }
}

fn load_graph<T: BufRead>(mut reader: T) -> HashMap<Vertex, Vec<Edge>> {
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let dim: Vec<usize> = buffer.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut graph = HashMap::new();
    for line in reader.lines().take(dim[1]).map(|s| s.unwrap()) {
        let edge: Vec<&str> = line.split_whitespace().collect();
        let v = Vertex::new(edge[0]);
        let e = Edge::new(edge[0], edge[1]);
        if !graph.contains_key(&v) {
            graph.insert(v, vec![e]);
        } else {
            graph.get_mut(&v).unwrap().push(e);
        }
    }
    graph
}
