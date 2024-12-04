use std::{collections::HashMap, io::{stdin, Read}};

fn main() {
	let mut data = String::new();
	stdin().read_to_string(&mut data).unwrap();
	let (left, right): (Vec<_>, Vec<_>) = data.lines()
		.map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>())
		.map(|nums| (nums[0], nums[1]))
		.unzip();

	let mut right_counts = HashMap::new();
	for r in right.iter() {
		*right_counts.entry(r).or_insert(0) += 1;
	}

	let similarity_score: i32 = left.iter().map(|num| num * right_counts.get(num).unwrap_or(&0)).sum();
	dbg!(similarity_score);
}