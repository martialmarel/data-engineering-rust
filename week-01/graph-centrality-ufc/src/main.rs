/*
Ask 1: What is the 'closeness centrality' in the context of this program, and how is it calculated?
Answer: Closeness centrality is a measure used in network theory (graph theory). It's a way of
detecting nodes that are able to spread information very efficiently through a graph.
The more central a node is, the closer it is to all other nodes in the network.
In the context of a program, it could be used in social network analysis, routing protocols, or
any other domain where you have a graph-like structure.

The closeness centrality of a node is calculated as the reciprocal of the sum of the length of the
shortest paths between the node and all other nodes in the graph. So, it's basically a measure of
the average length of the shortest paths from a node to all other nodes.

Ask 2: How does the add_edge function work, and why do you need to pass in an array of NodeIndex?
Answer: The function add_edge is a custom function designed to add an edge to a graph.
The graph is represented using the UnGraph data structure from the petgraph library.
UnGraph stands for "undirected graph", meaning that the edges have no direction: an edge
from node A to node B is the same as an edge from node B to node A.

The add_edge function takes four parameters:

    1. graph: This is a mutable reference to the graph where the edge will be added. The graph is
    of type UnGraph<&Fighter, f32>. This means that the nodes in the graph hold references to
    Fighter objects, and the edges have weights of type f32 (32-bit floating point numbers).

    2. nodes: This is a reference to an array of NodeIndex values. NodeIndex is a type from the
    petgraph library that represents the index of a node in a graph.

    3. a and b: These are indices for the nodes array. They represent the nodes that will be
    connected by the new edge.

Inside the function, the add_edge method of the UnGraph object is called. This method takes three
parameters: two NodeIndex values representing the nodes to connect, and a weight for the edge.
In this case, the nodes are nodes[a] and nodes[b], and the weight is 1.0.


Ask 3: Why do we calculate the degree of a node by counting its outgoing edges?
Answer: In graph theory, the degree of a node is the number of edges connected to it. For an
undirected graph, this simply means the number of edges attached to the node. However, in a
directed graph, the concept of degree is often split into the "in-degree" and the "out-degree".
The in-degree of a node is the number of edges coming into it, while the out-degree is the number
of edges going out of it.

Counting the outgoing edges to calculate the degree of a node is typically done when working with
directed graphs and you're interested in the out-degree specifically. This can be useful in many
scenarios, such as understanding the influence of a node in a social network (how many other nodes
it is directly connected to), analyzing web page links (how many other pages a page links to),
among others.
*/

use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}

impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

fn main() {
    let mut graph = UnGraph::new_undirected();

    let fighters = [
        Fighter::new("Dustin Poirier"),      // 0
        Fighter::new("Khabib Nurmagomedov"), // 1
        Fighter::new("Jose Aldo"),           // 2
        Fighter::new("Conor McGregor"),      // 3
        Fighter::new("Nate Diaz"),           // 4
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);

        // Betweenness centrality, not sure if it's correct
        // I think, it should be use https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#method.edges_connecting
        // let betweenness = graph
        //     .edges_connecting(node, )
        //     .count() as f32
        //     / (fighter_nodes.len() - 1) as f32;
        //println!("The betweenness centrality of {} is {:.2}", name, betweenness);

        // Explanation
        match name.as_str() {
            "Conor McGregor" => println!(
                "{} has the lowest centrality because he has fought with all other fighters in the network. In this context, a lower centrality value means a higher number of fights.",
                name
            ),
            "Dustin Poirier" | "Nate Diaz" => println!(
                "{} has a centrality of {:.2}, implying they had less fights compared to Conor McGregor but more than Khabib Nurmagomedov and Jose Aldo.",
                name, closeness
            ),
            "Khabib Nurmagomedov" | "Jose Aldo" => println!(
                "{} has the highest centrality of {:.2} as they have fought with the least number of fighters.",
                name, closeness
            ),
            _ => {}
        }
        println!("-----------------");
    }
}
