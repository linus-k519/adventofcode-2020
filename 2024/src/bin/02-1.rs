use std::io::{stdin, Read};

fn main() {
	let mut data = String::new();
	stdin().read_to_string(&mut data).unwrap();
	let safe_reports = data.lines()
		.map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>())
		.filter(|report| {
			let increasing = report.is_sorted_by(|&a, &b| is_safe_level_difference(a, b));
			let decreasing = report.is_sorted_by(|&a, &b| is_safe_level_difference(b, a));
			increasing || decreasing
		});

	dbg!(safe_reports.count());
}

fn is_safe_level_difference(a: i32, b: i32) -> bool {
	(a - b) >= 1 && (a - b) <= 3
}