use std::{collections::HashMap, fs::read_to_string, ops::AddAssign, path::Path};

use itertools::Itertools;

use crate::util::errors::MyError;

pub fn load(flname: impl AsRef<Path>) -> Result<HashMap<String, Vec<String>>, MyError> {
	let mut graph: HashMap<String, Vec<String>> = HashMap::new();
	for line in read_to_string(flname)?.lines() {
		let (label, children) = line.split_once(": ").ok_or("malformed input")?;
		graph
			.entry(label.to_string())
			.or_default()
			.extend(children.split(" ").map(String::from).collect_vec());
	}
	Ok(graph)
}

pub fn pathfind(graph: HashMap<String, Vec<String>>, source: &str, dest: &str) -> u128 {
	let mut paths_to_dest = 0;
	let mut frontier = HashMap::new();
	let mut next_frontier = HashMap::new();
	next_frontier.insert(source.to_string(), 1);

	while !next_frontier.is_empty() {
		frontier.drain();
		next_frontier.drain().for_each(|(k, v)| {
			frontier.insert(k, v);
		});
		// println!("{:?}", frontier);

		for (node, paths) in &frontier {
			if node == dest {
				paths_to_dest += paths;
			}
			if let Some(children) = graph.get(node) {
				for child in children {
					next_frontier.entry(child.clone()).or_default().add_assign(paths);
				}
			}
		}
	}
	paths_to_dest
}

pub fn p1(graph: HashMap<String, Vec<String>>) -> u128 {
	pathfind(graph, "you", "out")
}

pub fn p2(graph: HashMap<String, Vec<String>>) -> u128 {
	pathfind(graph.clone(), "svr", "fft")
		* pathfind(graph.clone(), "fft", "dac")
		* pathfind(graph.clone(), "dac", "out")
}
