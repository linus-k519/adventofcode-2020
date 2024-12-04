use std::io::{stdin, Read};

use regex::Regex;

fn main() {
	let mut puzzle_input = String::new();
	stdin().read_to_string(&mut puzzle_input).unwrap();
	let command_regex = Regex::new(r"(mul|do|don't)\((?:(\d+),(\d+))?\)").unwrap();

	let mut multiplication_enabled = true;
	let multiplication_sum: i64 = command_regex.captures_iter(&puzzle_input)
		.filter_map(|multiplication_capture| {
			let command_name = multiplication_capture.get(1).unwrap().as_str();
			match command_name {
				"mul" if multiplication_enabled => {
					let a: i64 = multiplication_capture.get(2).unwrap().as_str().parse().unwrap();
					let b: i64 = multiplication_capture.get(3).unwrap().as_str().parse().unwrap();
					Some(a * b)
				},
				"do" => {
					multiplication_enabled = true;
					None
				},
				"don't" => {
					multiplication_enabled = false;
					None
				},
				_ => None,
			}
		})
		.sum();
	dbg!(multiplication_sum);
}