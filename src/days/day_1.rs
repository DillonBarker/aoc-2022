use crate::helpers::open_file;

use std::io::prelude::*;

pub fn solve() {
	match open_file("day_1") {
		Ok(reader) => {
			let lines: Vec<String> = reader.lines().
				map(|item| item.unwrap()).
				collect();
			solve_a(lines);
		},
		Err(e) => {
			panic!("crash and burn: {}", e);
		}
	};

	match open_file("day_1") {
		Ok(reader) => {
			let lines: Vec<String> = reader.lines().
				map(|item| item.unwrap()).
				collect();
			solve_b(lines);
		},
		Err(e) => {
			panic!("crash and burn: {}", e);
		}
	};
}

fn solve_a(lines: Vec<String>) -> i32  {
	let mut highest_cal: i32 = 0;
	
	let mut current_cal: i32 = 0;
	for line in lines {
		if line != "" {
			current_cal = current_cal + line.parse::<i32>().unwrap();
		}

		if line == "" {
			current_cal = 0;
		}

		if current_cal > highest_cal {
			highest_cal = current_cal;
		}
	}

	println!("Day 1 part a: {}", highest_cal);
	return highest_cal
}

fn solve_b(lines: Vec<String>) -> i32  {
	let mut highest_cal: i32 = 0;
	let mut second_highest_cal: i32 = 0;
	let mut third_highest_cal: i32 = 0;

	let mut current_cal: i32 = 0;
	for line in lines {
		

		if line != "" {
			current_cal = current_cal + line.parse::<i32>().unwrap();
		}

		if line == "" {
			if current_cal > highest_cal {
				third_highest_cal = second_highest_cal;
				second_highest_cal = highest_cal;
				highest_cal = current_cal;
			} else if current_cal > second_highest_cal { 
				third_highest_cal = second_highest_cal;
				second_highest_cal = current_cal;
			} else if current_cal > third_highest_cal {
				third_highest_cal = current_cal;
			}
			
			current_cal = 0;
		}
	}
	let total_cal = highest_cal + second_highest_cal + third_highest_cal;
	println!("Day 1 part b: {}", total_cal);
	return total_cal;
}
