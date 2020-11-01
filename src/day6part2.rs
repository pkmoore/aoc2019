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
use petgraph::algo::astar;

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

    // We are going to use astar to find our path so we need nodes going in
    // both directions.  Loop through all of our edges and add ones going
    // the opposite direction to our path.
    // We can't just reverse all the edges here because we will need to use
    // an edge going the original direction in some cases.
    for i in &edges {
        let (orbiter, orbitee) = i;
        let orbiter_index = *nodes_map.get(orbiter).unwrap();
        let orbitee_index = *nodes_map.get(orbitee).unwrap();
        //We need edges going both ways to get a path to santa
        nodes_graph.add_edge(orbitee_index, orbiter_index, 1);
    }

    let you = nodes_map.get("YOU").unwrap();
    let santa = nodes_map.get("SAN").unwrap();

    // We aren't using the actual path so drop it
    let (length, _) = astar(
        &nodes_graph,
        *you,
        // Our recognize our destination when we reach
        // a node == santa's NodeIndex
        |x| x == *santa,
        // All edges are weighted 1
        |_| 1,
        // Don't use an estimate to optimize
        |_| 0,
    )
    .unwrap();

    // Subtract 2 because because the answer they want is from the planet
    // YOU is orbiting to the planet SAN is orbiting.  We need to not count the
    // edge from these terminal planets to the YOU and SAN nodes
    println!("Orbital changes required: {}", length - 2);

    Ok(())
}
