use std::io::{stdin, Read};

fn main() {
	let mut puzzle_input = String::new();
	stdin().read_to_string(&mut puzzle_input).unwrap();

	let matrix: Vec<Vec<char>> = puzzle_input.lines().map(|line| line.chars().collect()).collect();
	let mut xmas_counter: usize = 0;
	for y in 0..matrix.len() {
		for x in 0..matrix[y].len() {
			xmas_counter += [
				// Diagonal
				y + 3 < matrix.len() && x + 3 < matrix[x].len() && matrix[y][x] == 'X' && matrix[y + 1][x + 1] == 'M' && matrix[y + 2][x + 2] == 'A' && matrix[y + 3][x + 3] == 'S',
				y as isize - 3 >= 0 && x as isize - 3 >= 0 && matrix[y][x] == 'X' && matrix[y - 1][x - 1] == 'M' && matrix[y - 2][x - 2] == 'A' && matrix[y - 3][x - 3] == 'S',
				y + 3 < matrix.len() && x as isize - 3 >= 0 && matrix[y][x] == 'X' && matrix[y + 1][x - 1] == 'M' && matrix[y + 2][x - 2] == 'A' && matrix[y + 3][x - 3] == 'S',
				y as isize - 3 >= 0 && x + 3 < matrix[x].len() && matrix[y][x] == 'X' && matrix[y - 1][x + 1] == 'M' && matrix[y - 2][x + 2] == 'A' && matrix[y - 3][x + 3] == 'S',
				// Horizontal
				x + 3 < matrix[y].len() && matrix[y][x] == 'X' && matrix[y][x + 1] == 'M' && matrix[y][x + 2] == 'A' && matrix[y][x + 3] == 'S',
				x as isize - 3 >= 0 && matrix[y][x] == 'X' && matrix[y][x - 1] == 'M' && matrix[y][x - 2] == 'A' && matrix[y][x - 3] == 'S',
				// Vertical
				y + 3 < matrix.len() && matrix[y][x] == 'X' && matrix[y + 1][x] == 'M' && matrix[y + 2][x] == 'A' && matrix[y + 3][x] == 'S',
				y as isize - 3 >= 0 && matrix[y][x] == 'X' && matrix[y - 1][x] == 'M' && matrix[y - 2][x] == 'A' && matrix[y - 3][x] == 'S',
			].into_iter().map(|condition| condition as usize).sum::<usize>();
		}
	}
	dbg!(&xmas_counter);
}