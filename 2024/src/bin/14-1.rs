//! NOT WORKING YET

use std::{io::{stdin, Read}, rc::Rc};

use regex::Regex;

fn main() {
    let mut puzzle_input = String::new();
    stdin().read_to_string(&mut puzzle_input).unwrap();

	/* let grid_size = Rc::new(Point2d {
		x: 101,
		y: 103,
	}); */
	let grid_size = Rc::new(Point2d {
		x: 13,
		y: 7,
	});

    let mut robots = puzzle_input
        .lines()
        .map(|line| Robot::new(line, Rc::clone(&grid_size)))
        .collect::<Vec<_>>();

    // Referred to as "seconds" in the task description
    const AMOUNT_OF_ROUNDS: i32 = 100;
    // Move robots
    for _round in 0..AMOUNT_OF_ROUNDS {
        for robot in robots.iter_mut() {
            robot.update_position();
        }
    }

	dbg!(&robots);

	let safety_factor = calculate_safety_factor(&robots);
    dbg!(safety_factor);
}

#[derive(Debug, Clone)]
struct Robot {
	/// The current position of the robot in the grid.
    position: Point2d<i32>,
	/// Dimensions of the grid the robot is moving in, shared between all robots.
    grid_size: Rc<Point2d<i32>>,
	/// A vector of the robot's movement relative to its current position.
    velocity: Point2d<i32>,
}

#[derive(Debug, Copy, Clone)]
struct Point2d<T> {
    x: T,
    y: T,
}

impl Robot {
	/// Parses a robot from a string, e.g. "p=0,4 v=3,-3".
	/// 
	/// # Panics
	/// Panics if the input string does not match the expected format.
    fn new(robot: &str, grid_size: Rc<Point2d<i32>>) -> Self {
        let robot_regex = Regex::new(r#"p=(\d+),(\d+) v=(-?\d+),(-?\d)"#).unwrap();
        let robot_capture = robot_regex.captures(robot).unwrap();
        Self {
            position: Point2d {
                x: robot_capture[1].parse().unwrap(),
                y: robot_capture[2].parse().unwrap(),
            },
            grid_size,
            velocity: Point2d {
                x: robot_capture[3].parse().unwrap(),
                y: robot_capture[4].parse().unwrap(),
            },
        }
    }

    /// Moves the robot's [position](Self::position) with its [velocity](Self::velocity) with wrap-around according to [grid size](Self::grid_size).
    fn update_position(&mut self) {
        self.position.x = (self.position.x + self.velocity.x).rem_euclid(self.grid_size.x);
        self.position.y = (self.position.y + self.velocity.y).rem_euclid(self.grid_size.y);
		assert!(self.position.x >= 0 && self.position.x < self.grid_size.x);
		assert!(self.position.y >= 0 && self.position.y < self.grid_size.y);
    }
}

fn calculate_safety_factor<'a>(robots: &[Robot]) -> usize {
	let quadrant_top_left = robots
		.iter()
		.filter(|robot| robot.position.x < robot.grid_size.x / 2 && robot.position.y < robot.grid_size.y / 2)
		.count();
	let quadrant_top_right = robots
		.iter()
		.filter(|robot| robot.position.x > robot.grid_size.x / 2 && robot.position.y < robot.grid_size.y / 2)
		.count();
	let quadrant_bottom_left = robots
		.iter()
		.filter(|robot| robot.position.x < robot.grid_size.x / 2 && robot.position.y > robot.grid_size.y / 2)
		.count();
	let quadrant_bottom_right = robots
		.iter()
		.filter(|robot| robot.position.x > robot.grid_size.x / 2 && robot.position.y > robot.grid_size.y / 2)
		.count();

	dbg!(quadrant_top_left);
	dbg!(quadrant_top_right);
	dbg!(quadrant_bottom_left);
	dbg!(quadrant_bottom_right);

	quadrant_top_left * quadrant_top_right * quadrant_bottom_left * quadrant_bottom_right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_robot() {
		let grid_size = Rc::new(Point2d {
			x: 11,
			y: 7,
		});
        let robot = Robot::new("p=0,4 v=3,-3", grid_size);
        assert_eq!(robot.position.x, 0);
        assert_eq!(robot.position.y, 4);
        assert_eq!(robot.velocity.x, 3);
        assert_eq!(robot.velocity.y, -3);
    }

    #[test]
    fn test_update_robot_position() {
		let grid_size = Rc::new(Point2d {
			x: 11,
			y: 7,
		});
        let mut robot = Robot::new("p=2,4 v=2,-3", grid_size);

        // After 1 second
        robot.update_position();
        assert_eq!(robot.position.x, 4);
        assert_eq!(robot.position.y, 1);

        // After 2 seconds
        robot.update_position();
        assert_eq!(robot.position.x, 6);
        assert_eq!(robot.position.y, 5);

        // After 3 seconds
        robot.update_position();
        assert_eq!(robot.position.x, 8);
        assert_eq!(robot.position.y, 2);

        // After 4 seconds
        robot.update_position();
        assert_eq!(robot.position.x, 10);
        assert_eq!(robot.position.y, 6);

        // After 5 seconds
        robot.update_position();
        assert_eq!(robot.position.x, 1);
        assert_eq!(robot.position.y, 3);
    }
}
