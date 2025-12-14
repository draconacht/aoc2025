use std::{
	fmt::Display,
	iter::Sum,
	ops::{Add, AddAssign, Div, Mul, MulAssign, Neg},
};

use itertools::Itertools;


pub fn gcd(mut n: i128, mut m: i128) -> i128 {
	assert!(n != 0 && m != 0);
	while m != 0 {
		if m < n {
			std::mem::swap(&mut m, &mut n);
		}
		m %= n;
	}
	n
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Fraction(pub i128, pub i128);

impl PartialOrd for Fraction {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		(*self + (-*other)).0.partial_cmp(&0i128)
	}
}

impl Sum for Fraction {
	fn sum<I>(iter: I) -> Self
	where
		I: Iterator<Item = Self>,
	{
		iter.fold(Fraction(0, 0), Self::add)
	}
}

impl Neg for Fraction {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self(-self.0, self.1)
	}
}

impl Fraction {
	#[inline]
	fn reduce(&mut self) {
		if self.0 == 0 {
			self.1 = 0;
			return;
		}
		let gcd = gcd(self.0.abs(), self.1.abs());
		let m = self.0.signum() * self.1.signum();
		self.0 = self.0.abs() * m / gcd;
		self.1 = self.1.abs() / gcd;
	}

	pub fn is_integer(&self) -> bool {
		self.1 == 0 || self.1 == 1
	}
}

impl Display for Fraction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.1 > 1 {
			f.write_str(format!("{}|{}", self.0, self.1).as_str())
		} else {
			f.write_str(format!("{}", self.0).as_str())
		}
	}
}

impl Mul for Fraction {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let mut out = Self(self.0 * rhs.0, self.1 * rhs.1);
		out.reduce();
		out
	}
}
impl Div for Fraction {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		assert!(rhs.0 != 0);
		let mut out = Self(self.0 * rhs.1, self.1 * rhs.0);
		out.reduce();
		out
	}
}

impl MulAssign for Fraction {
	fn mul_assign(&mut self, rhs: Self) {
		self.0 *= rhs.0;
		self.1 *= rhs.1;
		self.reduce();
	}
}
impl AddAssign for Fraction {
	fn add_assign(&mut self, rhs: Self) {
		if rhs == Fraction(0, 0) {
			return;
		}
		if *self == Fraction(0, 0) {
			self.0 = rhs.0;
			self.1 = rhs.1;
			return;
		}
		self.0 = (self.0 * rhs.1) + (self.1 * rhs.0);
		self.1 *= rhs.1;
		self.reduce();
	}
}

impl Add for Fraction {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		if rhs == Fraction(0, 0) {
			return self;
		}
		if self == Fraction(0, 0) {
			return rhs;
		}
		let mut out = Self((self.0 * rhs.1) + (self.1 * rhs.0), self.1 * rhs.1);
		out.reduce();
		out
	}
}

pub struct Matrix(pub Vec<Vec<Fraction>>);

#[derive(Clone, Debug)]
pub enum Op {
	Swap(usize, usize),
	Mult(usize, Fraction),
	Add(usize, usize, Fraction),
}

impl Matrix {
	pub fn col(&self, i: usize) -> Vec<Fraction> {
		self.0.iter().map(|x| x[i]).collect_vec()
	}
	pub fn apply(&mut self, op: Op) -> Op {
		match op {
			Op::Swap(i, j) => self.0.swap(i, j),
			Op::Mult(i, k) => self.0[i].iter_mut().for_each(|x| *x *= k),
			Op::Add(i, j, m) => (0..self.0[0].len()).for_each(|k| {
				let toadd = m * self.0[j][k];
				self.0[i][k] += toadd;
			}),
		}
		// println!("{:?}", op);
		// let r = self.0.clone();
		// println!("{}", Grid(r));
		op
	}

	fn swap_pilot(&mut self, (y, x): (usize, usize)) -> Option<Op> {
		for k in y..self.0.len() {
			if self.0[k][x] != Fraction(0, 0) {
				return Some(self.apply(Op::Swap(y, k)));
			}
		}
		None
	}

	fn reduce_to_zero(&mut self, (y, x): (usize, usize), free_vars: usize) -> Vec<Op> {
		// println!("{} {} => 0", y, x);
		let mut ops = vec![];
		let m = &mut self.0;
		if m[y][x] == Fraction(0, 0) {
			return ops;
		}
		let pilot = (x - free_vars, x); // guaranteed to be non zero
		let factor = -m[y][x] / m[pilot.0][pilot.1];
		ops.push(Op::Add(y, pilot.0, factor));
		self.apply(ops[0].clone());
		ops
	}

	fn reduce_to_one(&mut self, (y, x): (usize, usize)) -> Option<Vec<Op>> {
		// println!("{} {} => 1", y, x);
		let mut ops = vec![];
		if self.0[y][x] == Fraction(0, 0) {
			ops.push(self.swap_pilot((y, x))?);
		}
		if self.0[y][x] == Fraction(1, 1) {
			return Some(ops);
		}
		let factor = Fraction(1, 1) / self.0[y][x];
		ops.push(Op::Mult(y, factor));
		self.apply(ops[ops.len() - 1].clone());
		Some(ops)
	}

	pub fn reduce(&mut self) -> (Vec<Op>, Vec<usize>) {
		let m = &mut self.0;
		let mut free_vars = vec![];
		let mut ops = vec![];
		let (y_max, x_max) = (m.len(), m[0].len());
		for x in 0..x_max {
			if x - free_vars.len() >= y_max {
				free_vars.push(x);
				continue;
			}
			let op = &mut self.reduce_to_one((x - free_vars.len(), x));
			if let Some(o) = op {
				ops.append(o);
			} else {
				free_vars.push(x);
				// println!("{:?}", free_vars);
				continue;
			}
			for y in 0..y_max {
				if x != y + free_vars.len() {
					ops.append(&mut self.reduce_to_zero((y, x), free_vars.len()));
				}
			}
		}
		(ops, free_vars)
	}
}
