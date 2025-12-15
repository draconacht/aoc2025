use std::{fs::read_to_string, path::Path};

use itertools::Itertools;
use regex::Regex;

use crate::util::{
	errors::MyError,
	grid::Grid,
	math::{Fraction, Matrix},
};

#[derive(Debug)]
pub struct Input {
	target_lights: Vec<bool>,
	switches: Vec<Vec<usize>>,
	joltages: Vec<u32>,
}

fn parse(inp: &str) -> Result<Input, MyError> {
	// TODO - redo these with nom later (?)
	let re = Regex::new(r"^\[(.*?)\] \((.*)\) \{(.*?)\}$")?;
	let captures = re.captures(inp).ok_or("malformed input")?;
	// println!("{:?}", captures);
	let target_lights = captures[1].chars().map(|x| x == '#').collect_vec();
	let switches = captures[2]
		.split(") (")
		.map(|x| x.split(",").map(|ch| ch.parse::<usize>()).try_collect())
		.try_collect()?;
	let joltages = captures[3].split(",").map(|ch| ch.parse::<u32>()).try_collect()?;

	Ok(Input {
		target_lights,
		switches,
		joltages,
	})
}

pub fn load(flname: impl AsRef<Path>) -> Result<Vec<Input>, MyError> {
	read_to_string(flname)?.lines().map(parse).try_collect()
}

fn apply(displays: &[bool], switches: &[Vec<usize>], actions: &[bool]) -> Vec<bool> {
	let mut displays_final = Vec::from(displays);
	for (s, a) in switches.iter().zip(actions) {
		for switch in s {
			if !a {
				continue;
			};
			displays_final[*switch] = !displays_final[*switch];
		}
	}
	// println!("{:?} {:?}", actions, displays_final);
	displays_final
}

fn mk_bitarray(max_len: usize, sparse_indices: &[usize]) -> Vec<bool> {
	let mut toret = vec![false; max_len];
	for k in sparse_indices {
		toret[*k] = true;
	}
	toret
}

fn p1_solve(inp: &Input) -> u32 {
	let (k, l) = (inp.target_lights.len(), inp.switches.len());
	for trves in 1..l {
		if (0..l)
			.permutations(trves)
			.map(|n| mk_bitarray(l, &n))
			.map(|p| apply(&vec![false; k], &inp.switches, &p))
			.any(|x| x == inp.target_lights)
		{
			return trves as u32;
		}
	}
	panic!("invalid puzzle ?")
}

pub fn p1(inp: &[Input]) -> u32 {
	inp.iter().map(p1_solve).sum()
}

fn p2_solve(inp: &Input) -> u32 {
	let l_switches = inp.switches.len();
	let mut m = vec![];
	for i in 0..inp.joltages.len() {
		let mut row = vec![Fraction(0, 0); l_switches];
		(0..l_switches).for_each(|j| {
			row[j] = if inp.switches[j].contains(&i) {
				Fraction(1, 1)
			} else {
				Fraction(0, 0)
			};
		});
		m.push(row)
	}

	let mut augment_mx = Matrix(inp.joltages.iter().map(|x| vec![Fraction(*x as i128, 1)]).collect_vec());
	// println!("======================");
	// println!("{}", Grid(m.clone()));
	let mut mx = Matrix(m);
	let (ops, free_vars) = mx.reduce();
	// println!("{}", Grid(mx.0.clone()));
	// println!("ops: {:?}", ops);
	for op in ops {
		augment_mx.apply(op);
	}
	let augment = augment_mx.col(0);
	// println!("augment\n{}", Grid(augment_mx.0.clone()));
	// println!("free vars: {:?}", free_vars);

	let cost: Fraction = augment.clone().into_iter().sum();
	let mut min_cost = 0u32;
	let max_joltage = *inp.joltages.iter().max().unwrap() as i128;

	if free_vars.is_empty() {
		return cost.0 as u32;
	} else if free_vars.len() == 1 {
		min_cost = 1_000_000;
		'outer: for j in 0..max_joltage {
			let mut vals = vec![];

			for k in 0..mx.0.len() {
				let v = augment[k] + Fraction(-j, 1) * mx.0[k][free_vars[0]];
				if v < Fraction(0, 0) || !v.is_integer() {
					continue 'outer;
				}
				vals.push(v);
			}

			let cost = (vals.clone().into_iter().sum::<Fraction>().0 + j) as u32;
			if cost < min_cost {
				min_cost = cost;
			}
		}
	} else if free_vars.len() == 2 {
		min_cost = 1_000_000;
		for j0 in 0..max_joltage {
			'outer: for j1 in 0..(max_joltage - j0) {
				let mut vals = vec![];

				for k in 0..mx.0.len() {
					let v = augment[k]
						+ Fraction(-j0, 1) * mx.0[k][free_vars[0]]
						+ Fraction(-j1, 1) * mx.0[k][free_vars[1]];
					if v < Fraction(0, 0) || !v.is_integer() {
						continue 'outer;
					}
					vals.push(v);
				}

				let cost = (vals.clone().into_iter().sum::<Fraction>().0 + j0 + j1) as u32;
				if cost < min_cost {
					min_cost = cost;
				}
			}
		}
	} else if free_vars.len() == 3 {
		min_cost = 1_000_000;
		for j0 in 0..max_joltage {
			for j1 in 0..(max_joltage - j0) {
				'outer: for j2 in 0..(max_joltage - j0 - j1) {
					let mut vals = vec![];

					for k in 0..mx.0.len() {
						let v = augment[k]
							+ Fraction(-j0, 1) * mx.0[k][free_vars[0]]
							+ Fraction(-j1, 1) * mx.0[k][free_vars[1]]
							+ Fraction(-j2, 1) * mx.0[k][free_vars[2]];
						if v < Fraction(0, 0) || !v.is_integer() {
							continue 'outer;
						}
						vals.push(v);
					}

					let cost = (vals.clone().into_iter().sum::<Fraction>().0 + j0 + j1 + j2) as u32;
					if cost < min_cost {
						min_cost = cost;
					}
				}
			}
		}
	} else {
		panic!("shit")
	}
	// println!("{}", min_cost);
	min_cost
}

pub fn p2(inp: &[Input]) -> u32 {
	inp.iter().map(p2_solve).sum()
}
