use std::io::{stdin, Read};

use regex::Regex;

fn main() {
	let mut puzzle_input = String::new();
	stdin().read_to_string(&mut puzzle_input).unwrap();
	let multiplication_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
	let multiplication_sum: i64 = multiplication_regex.captures_iter(&puzzle_input)
		.map(|multiplication_capture| {
			let a: i64 = multiplication_capture.get(1).unwrap().as_str().parse().unwrap();
			let b: i64 = multiplication_capture.get(2).unwrap().as_str().parse().unwrap();
			a * b
		}).sum();
	dbg!(multiplication_sum);
}