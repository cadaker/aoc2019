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

type PGraph = HashMap<String, String>;

fn parent_graph(graph: &Graph) -> PGraph {
    let mut ret = PGraph::new();
    for (node, children) in graph {
        for child in children {
            ret.insert(child.clone(), node.clone());
        }
    }
    ret
}

fn parents(graph: &PGraph, node: &String) -> Vec<String> {
    let mut parents: Vec<String> = vec![];
    let mut n = node.clone();
    loop {
        match graph.get(&n) {
            Some(parent) => {
                n = parent.clone();
                parents.push(n.clone());
            },
            None => return parents
        }
    }
}

fn index_of(haystack: &Vec<String>, needle: &String) -> Option<usize> {
    let lookup = haystack
        .iter()
        .enumerate()
        .find(|(_, s)| *s == needle);
    Some(lookup?.0)
}

fn common_ancestor_lengths(graph: &PGraph, node0: &String, node1: &String) -> Option<(usize,usize)> {
    let parents0 = parents(graph, node0);
    let parents1 = parents(graph, node1);
    for parent0 in &parents0 {
        match index_of(&parents1, parent0) {
            Some(ix1) => {
                let ix0 = index_of(&parents0, parent0).unwrap();
                return Some((ix0, ix1))
            },
            None => continue
        }
    }
    None
}

fn main() {
    let graph = parse_graph();

    println!("{}", total_orbits(&graph));
    let x = common_ancestor_lengths(
        &parent_graph(&graph),
        &String::from("YOU"),
        &String::from("SAN"));
    let (len0,len1) = x.unwrap();
    println!("{}", len0+len1);
}
