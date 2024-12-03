use std::{io::{stdin, Read}, iter};

fn main() {
	let mut data = String::new();
	stdin().read_to_string(&mut data).unwrap();
	let (mut left, mut right): (Vec<_>, Vec<_>) = data.lines()
		.map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>())
		.map(|nums| (nums[0], nums[1]))
		.unzip();
	left.sort();
	right.sort();

	let total_distance: i32 = iter::zip(left.iter(), right.iter()).map(|(l, r)| i32::abs(l - r)).sum();
	dbg!(total_distance);
}