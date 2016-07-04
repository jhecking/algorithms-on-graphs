use std::env;
use std::fs::File;
use std::io::BufReader;

mod graph;
mod tuple_reader;

use tuple_reader::TupleReader;

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
    match command.as_ref() {
        "reach" => {
            let graph = graph::Graph::load(&mut reader, false);
            let (from, to) = reader.next_tuple();
            println!("Checking reachability {} -> {}: {}", from, to, graph.is_reachable(from, to));
        },
        "comp" => {
            let graph = graph::Graph::load(&mut reader, false);
            let comps = graph.connected_components();
            println!("Connected components: {:?}", comps);
        },
        "print" => {
            let graph = graph::Graph::load(&mut reader, false);
            println!("{:?}", graph)
        },
        _ => println!("Unknown command: {}", command)
    }
}
