use std::collections::{HashMap, HashSet};

use crate::day_8::SubsetGenerator;
use crate::error::Result;
use crate::input::Input;
use crate::util::grid::Grid;
use crate::{day, day_tests};

type Node = u16;

struct Network {
    edges: HashMap<Node, Vec<Node>>, // adjacency map
    matrix: Grid<bool>,              // incidence matrix for quick edges lookup
}

impl Network {
    fn new(input: Input) -> Self {
        let max_nodes: usize = 26 * 26;

        let mut network_matrix = Grid::<bool>::with_size(max_nodes, max_nodes, false);
        let mut network_edges = HashMap::<Node, Vec<Node>>::new();

        input.lines().for_each(|line| {
            let line = line.expect("valid input");
            let mut parts = line.trim_end().split("-");

            if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
                let a = Self::label_to_id(a);
                let b = Self::label_to_id(b);

                network_edges.entry(a).or_insert_with(|| vec![]).push(b);
                network_edges.entry(b).or_insert_with(|| vec![]).push(a);

                network_matrix[(a, b)] = true;
                network_matrix[(b, a)] = true;
            }
        });

        Self {
            matrix: network_matrix,
            edges: network_edges,
        }
    }

    fn label_to_id(name: &str) -> Node {
        let id = name
            .chars()
            .fold(0, |acc, c| acc * 26 + (c as u16 - 'a' as u16));

        debug_assert!(name == Self::label_from_id(id), "id: {} name: {}", id, name);

        id
    }

    fn label_from_id(id: Node) -> String {
        let mut id = id;
        let mut name = String::new();

        for _ in 0..2 {
            name.push((id % 26 + 'a' as u16) as u8 as char);
            id /= 26;
        }
        name.chars().rev().collect()
    }

    // returns true if vertices form a complete subgraph
    fn is_clique(&self, vertices: &[Node]) -> bool {
        for i in 0..vertices.len() {
            for j in i + 1..vertices.len() {
                let v = vertices[i];
                let w = vertices[j];
                if !self.matrix[(v, w)] {
                    return false;
                }
            }
        }
        true
    }

    fn find_3cliques(&self) -> HashSet<Vec<Node>> {
        let mut result = HashSet::<Vec<Node>>::new();

        for (u, u_edges) in &self.edges {
            let mut pairs_gen = SubsetGenerator::new(2, u_edges.len());

            while let Some(pair) = pairs_gen.next() {
                let vertices = pair.iter().map(|x| u_edges[*x]).collect::<Vec<_>>();
                if self.is_clique(&vertices) {
                    let mut _3_clique = vertices;

                    _3_clique.push(*u);
                    _3_clique.sort();

                    result.insert(_3_clique);
                }
            }
        }
        result
    }

    fn find_max_clique(&self) -> Vec<Node> {
        let mut k = 3_u8; // we know 3-clique exists

        let mut max_clique = vec![];

        loop {
            let mut has_better_result = false;

            'vertices: for (u, u_edges) in &self.edges {
                if u_edges.len() < k as usize {
                    continue;
                }

                // generate all k-1 subsets of U's edges, and check if, together with U, they form a clique
                let mut gen = SubsetGenerator::new(k - 1, u_edges.len());

                while let Some(subset) = gen.next() {
                    let mut vertices = subset.iter().map(|x| u_edges[*x]).collect::<Vec<_>>();

                    if self.is_clique(&vertices) {
                        vertices.push(*u);

                        has_better_result = true;
                        max_clique = vertices;

                        break 'vertices; // we can stop here, we need just one proof for each k
                    }
                }
            }

            if has_better_result {
                println!("found a clique of size {}, trying {}...", k, k + 1);
                k += 1;
            } else {
                break;
            }
        }
        max_clique
    }

    // Task #1
    fn find_lan_parties(&self) -> usize {
        self.find_3cliques()
            .iter()
            .filter(|r| r.iter().any(|v| v / 26_u16 + 'a' as u16 == 't' as u16)) // any starting with 't'
            .count()
    }

    // Task #2
    fn find_largest_party(&self) -> usize {
        let mut max_clique = self.find_max_clique();
        max_clique.sort();

        print!("password:");
        for node in &max_clique {
            print!("{},", Self::label_from_id(*node));
        }
        println!();

        max_clique.len()
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let network = Network::new(input);

    let result = match part {
        day::Part::One => network.find_lan_parties(),
        day::Part::Two => network.find_largest_party(),
    } as i64;

    Ok(result)
}

day_tests!(
    "day_23-1.dat",
    1163,
    13 /* 'bm,bo,ee,fo,gt,hv,jv,kd,md,mu,nm,wx,xh' */
);
