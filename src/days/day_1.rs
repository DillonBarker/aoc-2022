use std::fs::File;
use std::io::{self, prelude::*, BufReader};


// const DAY: &str = "day_1";

pub fn solve() {
	solve_a();
	solve_b();
}

fn solve_a() -> io::Result<()>  {
	let file = File::open("inputs/day_1.txt")?;
	let reader = BufReader::new(file);

	let mut highest_cal: i32 = 0;

	let lines: Vec<String> = reader.lines().
		map(|item| item.unwrap()).
		collect();
	
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

	println!("{}", highest_cal);
	Ok(())
}

fn solve_b() -> io::Result<()>  {
	let file = File::open("inputs/day_1.txt")?;
	let reader = BufReader::new(file);

	let mut highest_cal: i32 = 0;
	let mut second_highest_cal: i32 = 0;
	let mut third_highest_cal: i32 = 0;

	let lines: Vec<String> = reader.lines().
		map(|item| item.unwrap()).
		collect();

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

	println!("{}", highest_cal + second_highest_cal + third_highest_cal);
	Ok(())
}
