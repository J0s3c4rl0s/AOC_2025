use std::fs::read_to_string;
// use petgraph::adj::{EdgeIndex, EdgeReference};
use petgraph::{Graph, Undirected};
use petgraph::graph::{self, Edge, EdgeIndex, UnGraph};
use petgraph::unionfind::UnionFind;
use petgraph::visit::EdgeRef;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy)]
struct Coord(f64, f64, f64);

fn main() {
    let input = read_to_string("input.txt")
        .expect("read file")
        .lines()
        .map(parse_to_tuple)
        .collect();
    pt2(input)
}

fn parse_to_tuple(line : &str) -> Coord {
    let line : Vec<f64> = line
        .split(",")
        .map(|coord_str| coord_str.parse().expect("Number")).collect();
    Coord(line[0], line[1], line[2])
}

fn pt2(input : Vec<Coord>) {
    let mut graph = UnGraph::<Coord, f64>::new_undirected();

    let node_indices: Vec<_> = input
        .iter()
        .map(|p| graph.add_node(p.to_owned()))
        .collect();
    
    for i in 0..node_indices.len() {
        for j in (i + 1)..node_indices.len() {
            let a = input[i];
            let b = input[j];
            let dist = euclidean(a, b);
            graph.add_edge(node_indices[i], node_indices[j], dist);
        }
    }


    let mut subgraphs: UnionFind<_> = UnionFind::new(node_indices.len());
    
    let mut edges : Vec<_> = graph.edge_references()
        .map(|e| (e.source(), e.target(), *e.weight()))
        .collect();

    edges.
        sort_by(|(_, _, w1), (_, _, w2)| w1.total_cmp(w2));


    let mut penultimate = edges[0].0;
    let mut last = edges[0].1;

    for (from, to, _) in edges {
        // Do union, if new update last two boxes 
        if subgraphs.union(from.index(), to.index()) {
            penultimate = from;
            last = to;
        }
    }

    let result = graph[penultimate].0 * graph[last].0;

    println!("Total is {result}")

}


fn pt1(input : Vec<Coord>) {
    let mut graph = UnGraph::<Coord, f64>::new_undirected();

    let node_indices: Vec<_> = input
        .iter()
        .map(|p| graph.add_node(p.to_owned()))
        .collect();
    
    for i in 0..node_indices.len() {
        for j in (i + 1)..node_indices.len() {
            let a = input[i];
            let b = input[j];
            let dist = euclidean(a, b);
            graph.add_edge(node_indices[i], node_indices[j], dist);
        }
    }


    let mut subgraphs: UnionFind<_> = UnionFind::new(node_indices.len());
    
    let mut edges : Vec<_> = graph.edge_references()
        .map(|e| (e.source(), e.target(), *e.weight()))
        .collect();

    edges.
        sort_by(|(_, _, w1), (_, _, w2)| w1.total_cmp(w2));


    for i in 0..1000 {
        // Select smallest edge 
        let (from, to, _) = edges[i];
        // Do union 
        subgraphs.union(from.index(), to.index());
    }

    // Find size of each subgraphs
    let mut sizes = vec![0; subgraphs.len()];
    for i in 0..subgraphs.len() {
        // increment by one each representative
        sizes[subgraphs.find(i)] += 1;
    }
    // sort sizes
    sizes.sort();
    // Multiply 3 largest subgraphs
    let result = sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3] ;

    println!("Total is {result}")   
}

fn smallest_edge(graph : Graph<Coord, f64, Undirected>) -> EdgeIndex {
    graph
    .edge_references()
    .min_by(|a, b| {
        a.weight()
            .partial_cmp(b.weight())
            .unwrap()
    }).unwrap().id()
}

fn euclidean(a: Coord, b: Coord) -> f64 {
    match (a, b) {
        (Coord(ax, ay, az), Coord(bx, by, bz)) => 
            ((ax - bx).powf(2.0) + (ay - by).powf(2.0) +(az - bz).powf(2.0)).sqrt()
    }
}