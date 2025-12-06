use itertools::Itertools;
use std::{fs::read_to_string, iter::zip, path::Path, str::FromStr};

use crate::util::errors::MyError;

#[derive(Debug, Clone)]
pub enum Operator {
	Add,
	Mult,
}

impl FromStr for Operator {
	type Err = MyError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"*" => Self::Mult,
			"+" => Self::Add,
			x => Err(format!("ew? {:}", x))?,
		})
	}
}

fn transpose<X: Copy>(inp: Vec<Vec<X>>) -> Vec<Vec<X>> {
	let (l0, l1) = (inp.len(), inp[0].len());
	(0..l1).map(|j| (0..l0).map(|i| inp[i][j]).collect()).collect()
}

fn transmogrify(inp: Vec<u32>) -> Vec<u32> {
	let x = transpose(
		inp.iter()
			.map(|x| format!("{:->4}", x).chars().collect_vec())
			.collect_vec(),
	);
	x.iter()
		.filter(|x| x.iter().any(|ch| ch.is_ascii_digit()))
		.map(|x| {
			x.iter()
				.collect::<String>()
				.replace("-", "")
				.replace("0", "")
				.parse::<u32>()
				.unwrap()
		})
		.collect_vec()
}

pub fn load(flname: impl AsRef<Path>) -> Result<Vec<(Vec<u32>, Operator)>, MyError> {
	let fl = read_to_string(flname)?.replace("\\", "");
	let rows = fl.lines();
	let (numeric_rows, op_rows): (Vec<_>, Vec<_>) =
		rows.partition(|x| x.trim().starts_with(|ch: char| ch.is_ascii_digit()));
	let data = numeric_rows
		.iter()
		.map(|x| {
			x.split_whitespace()
				.map(str::parse::<u32>)
				.collect::<Result<Vec<_>, _>>()
		})
		.collect::<Result<Vec<_>, _>>()?;
	let raw_ops = op_rows.first().ok_or("why")?.split_whitespace();
	let ops = raw_ops.map(Operator::from_str).collect::<Result<Vec<_>, _>>()?;
	Ok(zip(transpose(data), ops).collect())
}

pub fn load_p2(flname: impl AsRef<Path>) -> Result<Vec<(Vec<u32>, Operator)>, MyError> {
	let fl = read_to_string(flname)?.replace("\\", "");
	let rows = fl.lines();
	let (numeric_rows, op_rows): (Vec<_>, Vec<_>) =
		rows.partition(|x| x.trim().starts_with(|ch: char| ch.is_ascii_digit()));

	let op_row = op_rows[0];
	let starts = op_row
		.chars()
		.enumerate()
		.filter_map(|(i, x)| if x == ' ' { None } else { Some(i) })
		.collect::<Vec<_>>();
	let last_start = *starts.last().unwrap();

	let mut out_data: Vec<Vec<u32>> = vec![];
	for (curr, next) in starts.into_iter().tuple_windows::<(_, _)>() {
		out_data.push(
			numeric_rows
				.iter()
				.map(|x| u32::from_str(&String::from(*x)[curr..next - 1].replace(" ", "0")).unwrap())
				.collect(),
		)
	}
	out_data.push(
		numeric_rows
			.iter()
			.map(|x| u32::from_str(&String::from(*x)[last_start..].replace(" ", "0")).unwrap())
			.collect::<Vec<_>>(),
	);

	let raw_ops = op_rows.first().ok_or("why")?.split_whitespace();
	let ops = raw_ops.map(Operator::from_str).collect::<Result<Vec<_>, _>>()?;

	Ok(zip(out_data, ops).collect())
}

pub fn p1(inp: Vec<(Vec<u32>, Operator)>) -> u128 {
	let rows = inp.into_iter().map(|(row, op)| match op {
		Operator::Add => row.into_iter().fold(0u128, |a, b| a + b as u128),
		Operator::Mult => row.into_iter().fold(1u128, |a, b| a * b as u128),
	});
	rows.sum()
}

pub fn p2(inp: Vec<(Vec<u32>, Operator)>) -> u128 {
	let rows = inp.into_iter().map(|(row, op)| {
		let r = transmogrify(row);
		println!("{:?}", r);
		match op {
			Operator::Add => r.into_iter().fold(0u128, |a, b| a + b as u128),
			Operator::Mult => r.into_iter().fold(1u128, |a, b| a * b as u128),
		}
	});
	rows.sum()
}
