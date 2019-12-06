use std::collections::HashMap;
use std::io::{self, BufRead};

type Graph = HashMap<String, Vec<String>>;

fn parse_graph() -> Graph {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut graph: Graph = Graph::new();
    for line_r in handle.lines() {
        let line = &line_r.unwrap();
        if line.len() > 0 {
            let v: Vec<String> = line
                .split(")")
                .map(|part| String::from(part))
                .collect();
            graph.entry(v[0].clone()).or_default().push(v[1].clone());
        }
    }
    graph
}

fn total_orbits_dfs(graph: &Graph, node: &String, depth: u32) -> u32 {
    graph
        .get(node)
        .unwrap_or(&vec![])
        .iter()
        .map(|child| {
            total_orbits_dfs(graph, child, depth+1)
        })
        .sum::<u32>()
        + depth
}

fn total_orbits(graph: &Graph) -> u32 {
    total_orbits_dfs(graph, &String::from("COM"), 0)
}

fn main() {
    let graph = parse_graph();

    println!("{}", total_orbits(&graph));
}
