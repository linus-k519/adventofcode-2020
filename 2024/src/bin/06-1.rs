use std::io::{stdin, Read};


fn main() {
	let mut puzzle_input = String::new();
	stdin().read_to_string(&mut puzzle_input).unwrap();

	let mut map: Vec<Vec<char>> = puzzle_input.lines().map(|line| line.chars().collect()).collect();

	let start_position = map.iter()
		.enumerate() // Enumerate the y coordinate
		.find_map(|(y, row)| {
			row.iter()
				.position(|&cell| cell == '^')
				.map(|x| Position { x, y })
		})
		.expect("No start position found");

	// Remove the start position from the map
	map[start_position.y][start_position.x] = '.';

	let mut game = Game {
		map,
		position: start_position,
		direction: Direction::Up,
	};
	game.run();

	let visited_cells: usize = game.map.iter().flatten().map(|&cell| (cell == 'X') as usize).sum();
	dbg!(visited_cells);
}

struct Game {
	map: Vec<Vec<char>>,
	position: Position,
	direction: Direction,
}

impl Game {
	fn run(&mut self) {
		loop {
			self.map[self.position.y][self.position.x] = match self.map[self.position.y][self.position.x] {
				'.' => 'X', // Mark cell as visited
				'X' => 'X', // Cell was already visited; no state change
				'#' => panic!("Position on a blocked cell"),
				_ => panic!("Invalid character found in matrix"),
			};

			if !self.set_next_position_and_direction() {
				break;
			}
		}
	}

	/// Sets the next [position](Self::position) and [direction](Self::direction) by calling
	/// [calculate_next_position](Self::calculate_next_position) and checking if the next position
	/// is blocked. If so, turn right and try again for all directions.
	fn set_next_position_and_direction(&mut self) -> bool {
		const DIRECTIONS: usize = 4;
		for _ in 0..DIRECTIONS {
			let Some(next_position) = self.calculate_next_position() else { break };

			if self.map[next_position.y][next_position.x] == '#' {
				// Next position is blocked, turn right and try again
				self.direction = self.direction.turn_right();
				continue;
			}

			self.position = next_position;
			return true;
		}
		false
	}

	/// Advances [position](Self::position) in the current [direction](Self::direction).
	fn calculate_next_position(&mut self) -> Option<Position> {
		match self.direction {
			Direction::Up => {
				if self.position.y == 0 {
					return None;
				}
				Some(Position { x: self.position.x, y: self.position.y - 1 })
			},
			Direction::Right => {
				if self.position.x == self.map[self.position.y].len() - 1 {
					return None;
				}
				Some(Position { x: self.position.x + 1, y: self.position.y })
			},
			Direction::Bottom => {
				if self.position.y == self.map.len() - 1 {
					return None;
				}
				Some(Position { x: self.position.x, y: self.position.y + 1 })
			},
			Direction::Left => {
				if self.position.x == 0 {
					return None;
				}
				Some(Position { x: self.position.x - 1, y: self.position.y })
			},
		}
	}
}

#[derive(Debug, Clone)]
struct Position {
	x: usize,
	y: usize,
}

enum Direction {
	Up,
	Right,
	Bottom,
	Left,
}

impl Direction {
	fn turn_right(&self) -> Self {
		match self {
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Bottom,
			Direction::Bottom => Direction::Left,
			Direction::Left => Direction::Up,
		}
	}
}