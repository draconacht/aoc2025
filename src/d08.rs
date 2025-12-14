use std::{
	cmp::Reverse,
	collections::{BinaryHeap, HashMap},
	fs::read_to_string,
	path::Path,
	str::FromStr,
};

use crate::util::errors::MyError;
use itertools::Itertools;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Default, Debug, PartialOrd, Ord)]
pub struct Junction(u64, u64, u64);

#[derive(PartialEq, Eq, Debug)]
pub struct JunctionEdge(Junction, Junction);

impl PartialOrd for JunctionEdge {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}
impl Ord for JunctionEdge {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.1.distance(&self.0).cmp(&other.1.distance(&other.0))
	}
}

impl FromStr for Junction {
	type Err = MyError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split(",");
		let x = parts.next().ok_or("no x")?.parse::<u64>()?;
		let y = parts.next().ok_or("no y")?.parse::<u64>()?;
		let z = parts.next().ok_or("no z")?.parse::<u64>()?;
		Ok(Self(x, y, z))
	}
}

impl Junction {
	fn distance(&self, other: &Self) -> u64 {
		self.0.abs_diff(other.0).pow(2) + self.1.abs_diff(other.1).pow(2) + self.2.abs_diff(other.2).pow(2)
	}
}

pub fn load(flname: impl AsRef<Path>) -> Result<Vec<Junction>, MyError> {
	read_to_string(flname)?
		.lines()
		.map(Junction::from_str)
		.collect::<Result<Vec<_>, _>>()
}

pub fn p1(junctions: &[Junction], span_edges: usize) -> u32 {
	let mut smallest_edges: Vec<(&Junction, &Junction)> = junctions
		.iter()
		.tuple_combinations::<(_, _)>()
		.k_smallest_by_key(span_edges, |(a, b)| b.distance(a))
		.collect_vec();

	let mut graph: HashMap<Junction, (Vec<Junction>, bool)> = HashMap::new();
	for _ in 0..span_edges {
		let edge = smallest_edges.pop().unwrap();
		// println!("{:?}", edge);
		graph.entry(*edge.0).or_default().0.push(*edge.1);
		graph.entry(*edge.1).or_default().0.push(*edge.0);
	}

	let mut subgraphs: HashMap<Junction, Vec<Junction>> = HashMap::new();
	let init_graph = graph.clone();

	for root in init_graph.keys() {
		let mut frontier = vec![*root];
		while let Some(node) = frontier.pop() {
			let (neighbours, picked) = graph.get_mut(&node).unwrap();
			if *picked {
				continue;
			}
			subgraphs.entry(*root).or_default().push(node);
			*picked = true;
			frontier.append(&mut neighbours.clone())
		}
	}

	subgraphs.values().map(|val| val.len()).k_largest(3).product::<usize>() as u32
}

pub fn p2(junctions: &[Junction]) -> u64 {
	let edges = junctions
		.iter()
		.tuple_combinations::<(_, _)>()
		.map(|x| Reverse(JunctionEdge(*x.0, *x.1)));
	let mut heap = BinaryHeap::from_iter(edges);

	let mut root_of = HashMap::new();
	let mut children_of: HashMap<Junction, Vec<Junction>> = HashMap::new();
	let mut edge = heap.pop().unwrap().0;
	let mut n_edges = 1;
	root_of.insert(edge.0, edge.0);
	root_of.insert(edge.1, edge.0);
	children_of.entry(edge.0).or_default().push(edge.1);

	// let mut counted = 0;
	while n_edges < (junctions.len() - 1) {
		// counted += 1;
		// println!("{:?} {} {} {}", edge, n_edges, counted, edge.1.distance(&edge.0));
		edge = heap.pop().unwrap().0;

		if !root_of.contains_key(&edge.0) && !root_of.contains_key(&edge.1) {
			// new tree
			root_of.insert(edge.1, edge.0);
			root_of.insert(edge.0, edge.0);
			children_of.entry(edge.0).or_default().push(edge.1);
		} else if root_of.contains_key(&edge.0) && root_of.contains_key(&edge.1) {
			// both edges have a known root
			if root_of[&edge.0] == root_of[&edge.1] {
				// both are already in the same tree, so its a circular edge.
				continue;
			} else {
				// connecting two trees in the forest
				let winner_root = root_of[&edge.0];
				let loser_root = root_of[&edge.1];
				root_of.insert(loser_root, winner_root);
				children_of.entry(winner_root).or_default().push(loser_root);
				for child in children_of[&loser_root].clone().iter() {
					// winner steals the loser's children
					root_of.insert(*child, winner_root);
					children_of.entry(winner_root).or_default().push(*child)
				}
			}
		} else if root_of.contains_key(&edge.0) {
			// extend tree of 0 with 1
			root_of.insert(edge.1, root_of[&edge.0]);
			children_of.entry(root_of[&edge.0]).or_default().push(edge.1);
		} else if root_of.contains_key(&edge.1) {
			// extend tree of 1 with 0
			root_of.insert(edge.0, root_of[&edge.1]);
			children_of.entry(root_of[&edge.1]).or_default().push(edge.0);
		}
		n_edges += 1
	}

	edge.0.0 * edge.1.0
}
