use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

// reads pairs of numbers from a given input stream
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

// graph vertices are represented as integer numbers
type Vertex = u32;

// adjacency map contains a list of adjacent vertices for each vertex in the graph
type Adjacencies = HashMap<u32, Vec<u32>>;

// a graph consists of a list of edges
// TODO: how to represent vertices that do not have any edges?
#[derive(Debug)]
struct Graph {
    edges: Vec<(Vertex, Vertex)>
}

impl Graph {

    // loads a graph from an input stream:
    // first line contains the number of vertices v and edges e
    // next e lines contain pairs of vertices representing the edges of the graph
    fn load<T: TupleReader>(reader: &mut T) -> Graph {
        let (_, e) = reader.next_tuple();
        let mut edges = vec![];
        for _ in 0..e { 
            let edge = reader.next_tuple();
            edges.push(edge)
        }
        Graph { edges: edges }
    }

    // builds the adjacency map for the graph
    fn adjacencies(&self) -> Adjacencies {
        let mut adj: Adjacencies = HashMap::new();
        for edge in &self.edges {
            if !adj.contains_key(&edge.0) {
                adj.insert(edge.0, vec![]);
            }
            adj.get_mut(&edge.0).unwrap().push(edge.1);
        }
        adj
    }

    // depth first search of the graph starting at vertex v
    // marks each vertex visited during the search and returns the list of visited vertices
    fn explore(&self, v: Vertex) -> HashSet<Vertex> {
        fn visit(adj: &Adjacencies, visited: &mut HashSet<Vertex>, v: Vertex) {
            visited.insert(v);
            if let Some(adjacent) = adj.get(&v) {
                for w in adjacent {
                    if !visited.contains(w) {
                        visit(adj, visited, w.clone());
                    }
                }
            }
        }

        let adj = self.adjacencies();
        let mut visited = HashSet::new();
        visit(&adj, &mut visited, v);
        visited
    }

    // returns true if vertex w can be reached from vertex v
    fn is_reachable(&self, v: Vertex, w: Vertex) -> bool {
        let visited = self.explore(v);
        visited.contains(&w)
    }
}

fn main() {
    let filename = std::env::args().next().expect("Missing filename!");
    let file = File::open(filename).expect("Cannot open file!");
    let mut reader = BufReader::new(&file);
    let graph = Graph::load(&mut reader);
    println!("{:?}", graph);
    let (from, to) = reader.next_tuple();
    println!("Checking reachability {} -> {}: {}", from, to, graph.is_reachable(from, to));
}
