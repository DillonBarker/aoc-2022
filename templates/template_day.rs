use crate::helpers::open_file;

use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve() {
	match open_file("day_") {
		Ok(reader) => {
			solve_a(reader);
		},
		Err(e) => {
			panic!("crash and burn: {}", e);
		}
	};

	match open_file("day_") {
		Ok(reader) => {
			solve_b(reader);
		},
		Err(e) => {
			panic!("crash and burn: {}", e);
		}
	};
}

fn solve_a(reader: BufReader<File>) -> i32  {
	let num: i32 = 0;
	println!("Day 1 part a: {}", num);
	return num;
}

fn solve_b(reader: BufReader<File>) -> i32  {
	let num: i32 = 0;
	println!("Day 1 part b: {}", num);
	return num;
}


#[cfg(test)]
mod tests {
    use super::*;

		const INPUT: &str = "";

		#[test]
    fn solve_a_returns_count() {
			let result = solve_a(INPUT.to_string());
			assert_eq!(result, 7);
		}

		#[test]
    fn solve_b_returns_count() {
			let result = solve_b(INPUT.to_string());
			assert_eq!(result, 5);
		}
}
