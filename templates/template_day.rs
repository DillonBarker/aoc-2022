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

	println!("Day 1 part a: {}", highest_cal);
	return highest_cal
}

fn solve_b(lines: Vec<String>) -> i32  {
	let mut total_cal: i32 = 0;

	println!("Day 1 part b: {}", total_cal);
	return total_cal;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_a_returns_count() {
        let test_input: Vec<String> = vec!["1000".to_string(), "2000".to_string(), 
            "3000".to_string(), "".to_string(), "4000".to_string(), "".to_string(), 
            "5000".to_string(), "6000".to_string(), "".to_string(), "7000".to_string(), 
            "8000".to_string(), "9000".to_string(), "".to_string(), "10000".to_string()];

        let result = solve_a(test_input);
        assert_eq!(result, 24000);
    }

    #[test]
    fn solve_b_returns_count() {
        let test_input: Vec<String> = vec!["1000".to_string(), "2000".to_string(), 
            "3000".to_string(), "".to_string(), "4000".to_string(), "".to_string(), 
            "5000".to_string(), "6000".to_string(), "".to_string(), "7000".to_string(), 
            "8000".to_string(), "9000".to_string(), "".to_string(), "10000".to_string()];

        let result = solve_b(test_input);
        assert_eq!(result, 41000);
    }
}
