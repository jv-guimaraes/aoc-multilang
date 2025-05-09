use std::collections::{BTreeSet, HashMap};

use petgraph::{
    graph::{Graph},
    Undirected,
    algo::dijkstra,
    prelude::*,
};

fn load_graph<'a>() -> Graph<&'a str, i32, Undirected> {
    // let input = include_str!("..\\s_input.txt");
    let input = include_str!("..\\input.txt");
    let mut graph: Graph<&str, i32, Undirected> = Graph::default();
    let mut cities = HashMap::new();

    for line in input.lines() {
        let tokens = line.split(' ').collect::<Vec<_>>();
        let start = tokens[0];
        let destination = tokens[2];

        if !cities.contains_key(start) {
            cities.insert(start, graph.add_node(start));
        }
        if !cities.contains_key(destination) {
            cities.insert(destination, graph.add_node(destination));
        }

        let start = *cities.get(start).unwrap();
        let destination = *cities.get(destination).unwrap();
        let weight = tokens[4].parse::<i32>().unwrap();
        graph.add_edge(start, destination, weight);
    }
    graph
}

fn shortest_path(graph: &Graph<&str, i32, Undirected>, start: NodeIndex) -> i32 {
    let mut unvisited = BTreeSet::from_iter(graph.node_indices());
    unvisited.remove(&start);
    let mut total = 0;

    // loop
    let mut current_node = start;
    while !unvisited.is_empty() {
        let mut search = dijkstra(&graph, current_node, None, |e| *e.weight());
        search.remove(&current_node);
        search.retain(|k, _| unvisited.contains(k));
        // println!("search: {:?}", search);

        let mut next_node = search.iter().next().unwrap();
        for entry in search.iter() {
            if entry.1 < next_node.1 {
                next_node = entry;
            }
        }

        // println!("next: {:?}\n", next_node);
        total += next_node.1;
        unvisited.remove(next_node.0);
        current_node = *next_node.0;
    }
    total
}

fn longest_path(graph: &Graph<&str, i32, Undirected>, start: NodeIndex) -> i32 {
    let mut unvisited = BTreeSet::from_iter(graph.node_indices());
    unvisited.remove(&start);
    let mut current_node = start;

    // Loop
    let mut total = 0;
    while !unvisited.is_empty() {
        // println!("{:?}", unvisited);
        let edges = graph.edges(current_node).filter(|e| unvisited.contains(&e.target()));
        // println!("{:?}", edges.clone().map(|e| (e.target().index(), e.weight())).collect::<Vec<_>>());
        let next_edge = edges.max_by_key(|e| e.weight()).unwrap();
        total += next_edge.weight();
        let next_node = next_edge.target();
        // println!("next: {:?}", next_node.index());
        unvisited.remove(&next_node);
        current_node = next_node;
    }
    total
}

fn main() {
    let graph = load_graph();
    // println!("{:?}\n", graph);

    // part 1
    println!("Part 1: {:?}", graph.node_indices().map(|i| shortest_path(&graph, i)).min().unwrap());

    // part 2
    println!("Part 2: {:?}", graph.node_indices().map(|i| longest_path(&graph, i)).max().unwrap());
}
