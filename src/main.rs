use std::env;
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
type Adjacencies = HashMap<Vertex, Vec<Vertex>>;

// list of connected components
type ConnectedComponents = Vec<Vec<Vertex>>;

// a graph consists of a list of edges
// TODO: how to represent vertices that do not have any edges?
#[derive(Debug)]
struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<(Vertex, Vertex)>,
}

impl Graph {

    fn new(vertices: Vec<Vertex>, edges: Vec<(Vertex, Vertex)>) -> Graph {
        Graph { vertices: vertices, edges: edges }
    }

    // loads a graph from an input stream:
    // first line contains the number of vertices v and edges e
    // next e lines contain pairs of vertices representing the edges of the graph
    fn load<T: TupleReader>(reader: &mut T) -> Graph {
        let (v, e) = reader.next_tuple();
        let vertices = (1..v+1).collect();
        let mut edges = vec![];
        for _ in 0..e { 
            let edge = reader.next_tuple();
            edges.push(edge)
        }
        Graph::new(vertices, edges)
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

    // depth first search of the entire graph
    // returns the set of connected components
    fn depth_first_search(&self) -> ConnectedComponents {
        let mut components = vec![];
        let mut visited = HashSet::new();
        for v in &self.vertices {
            if !visited.contains(v) {
                let mut component = vec![];
                self.explore(v, &mut visited, &mut component);
                components.push(component);
            }
        }
        components
    }

    // depth first search of the graph starting at vertex v
    // marks each vertex visited during the search and returns the list of visited vertices
    fn explore(&self, v: &Vertex, visited: &mut HashSet<Vertex>, component: &mut Vec<Vertex>) {
        fn visit(v: &Vertex, adj: &Adjacencies, visited: &mut HashSet<Vertex>, component: &mut Vec<Vertex>) {
            visited.insert(v.clone());
            component.push(v.clone());
            if let Some(adjacent) = adj.get(v) {
                for w in adjacent {
                    if !visited.contains(w) {
                        visit(w, adj, visited, component);
                    }
                }
            }
        }

        let adj = &self.adjacencies();
        visit(&v, &adj, visited, component);
    }

    // returns true if vertex w can be reached from vertex v
    fn is_reachable(&self, v: Vertex, w: Vertex) -> bool {
        let mut visited = HashSet::new();
        let mut component = vec![];
        self.explore(&v, &mut visited, &mut component);
        visited.contains(&w)
    }
}

fn main() {
    if env::args().count() != 3 {
        let main = env::args().next().unwrap();
        println!("Usage: {} <command> <graph file>", main);
        std::process::exit(1);
    }
    let mut args = env::args();
    let command = args.nth(1).unwrap();
    let filename = args.next().unwrap();
    let file = File::open(filename).expect("Cannot open file!");
    let mut reader = BufReader::new(&file);
    let graph = Graph::load(&mut reader);
    match command.as_ref() {
        "reach" => {
            let (from, to) = reader.next_tuple();
            println!("Checking reachability {} -> {}: {}", from, to, graph.is_reachable(from, to));
        },
        "comp" => {
            let comps = graph.depth_first_search();
            println!("Connected components: {:?}", comps);
        },
        "print" => println!("{:?}", graph),
        _ => println!("Unknown command: {}", command)
    }
}
