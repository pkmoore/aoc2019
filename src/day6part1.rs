use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::collections::HashMap;

extern crate petgraph;
use petgraph::graph::DefaultIx;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
// Used for debugging purposes
//use petgraph::dot::{Dot, Config};
use petgraph::Direction;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day6.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    let mut nodes_map: HashMap<String, NodeIndex<DefaultIx>> = HashMap::new();
    let mut nodes_graph = DiGraph::<&String, i32>::new();

    let mut edges = Vec::<(String, String)>::new();
    while reader.read_line(&mut data)? > 0 {
        data.pop();
        let s = data.split(")").collect::<Vec<&str>>();
        let orbiter = s.get(0).unwrap().to_string();
        let orbitee = s.get(1).unwrap().to_string();
        edges.push((orbitee, orbiter));
        data.clear();
    }
    for i in &edges {
        let (orbiter, orbitee) = i;
        let orbiter_index;
        let orbitee_index;
        if !nodes_map.contains_key(orbiter) {
            orbiter_index = nodes_graph.add_node(orbiter);
            nodes_map.insert(orbiter.clone(), orbiter_index);
        } else {
            orbiter_index = *nodes_map.get(orbiter).unwrap();
        }

        if !nodes_map.contains_key(orbitee) {
            orbitee_index = nodes_graph.add_node(orbitee);
            nodes_map.insert(orbitee.clone(), orbitee_index);
        } else {
            orbitee_index = *nodes_map.get(orbitee).unwrap();
        }
        nodes_graph.add_edge(orbiter_index, orbitee_index, 1);
    }
    let mut total: u64 = 0;

    for key in nodes_map.keys() {
        total += count_orbits(&nodes_graph, *nodes_map.get(key).unwrap());
    }
    println!("Orbit checksum: {}", total);

    Ok(())
}

fn count_orbits(g: &DiGraph<&String, i32>, index: NodeIndex) -> u64 {
    let mut count: u64 = 0;
    let mut current_index = index;
    loop {
        let neighbor = g
            .neighbors_directed(current_index, Direction::Outgoing)
            .next();
        match neighbor {
            Some(n) => {
                current_index = n;
                count += 1;
            }
            None => {
                break;
            }
        }
    }
    count
}
