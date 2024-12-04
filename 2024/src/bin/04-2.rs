use std::io::{stdin, Read};

fn main() {
	let mut puzzle_input = String::new();
	stdin().read_to_string(&mut puzzle_input).unwrap();

	let matrix: Vec<Vec<char>> = puzzle_input.lines().map(|line| line.chars().collect()).collect();
	let mut xmas_counter: usize = 0;
	for y in 0..matrix.len() {
		for x in 0..matrix[y].len() {
			if !((y as isize - 1) >= 0 && (y + 1) < matrix.len() && (x as isize - 1) >= 0 && (x + 1) < matrix[y].len()) {
				continue;
			}
			let diagonal1 = (matrix[y-1][x-1] == 'M' && matrix[y][x] == 'A' && matrix[y+1][x+1] == 'S') || (matrix[y-1][x-1] == 'S' && matrix[y][x] == 'A' && matrix[y+1][x+1] == 'M');
			let diagonal2 = (matrix[y-1][x+1] == 'M' && matrix[y][x] == 'A' && matrix[y+1][x-1] == 'S') || (matrix[y-1][x+1] == 'S' && matrix[y][x] == 'A' && matrix[y+1][x-1] == 'M');
			xmas_counter += (diagonal1 && diagonal2) as usize;
		}
	}
	dbg!(&xmas_counter);
}