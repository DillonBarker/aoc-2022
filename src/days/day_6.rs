use crate::helpers::open_file;

use std::io::prelude::*;

pub fn solve() {
	match open_file("day_6") {
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

	match open_file("day_6") {
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
	let mut total: i32 = 1;

	for line in lines {
		let chars: Vec<&str> = line.split("").filter(|char| !char.is_empty()).collect();
		
		let mut counter: i32 = 0;
		let mut taken_chars: Vec<&str> = Vec::new();
		for char in chars {
			if taken_chars.contains(&char) {
				if taken_chars[0] == char {
					taken_chars.remove(0);
					taken_chars.push(char);
				} else if taken_chars[1] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.push(char);
				} else if taken_chars[2] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.push(char);
				} else {
					taken_chars.clear();
					taken_chars.push(char);
				}
			} else {
				taken_chars.push(char);
			}

			if taken_chars.len() == 4 {
				counter = counter + 1;
				println!("Day 6 part a: {}", counter);
				return counter
			}

			counter = counter + 1;
		}
	}

	println!("Day 1 part a: {}", total);
	return total
}

fn solve_b(lines: Vec<String>) -> i32  {
	let mut total: i32 = 1;

	for line in lines {
		let chars: Vec<&str> = line.split("").filter(|char| !char.is_empty()).collect();
		
		let mut counter: i32 = 0;
		let mut taken_chars: Vec<&str> = Vec::new();
		for char in chars {
			if taken_chars.contains(&char) {
				if taken_chars[0] == char {
					taken_chars.remove(0);
					taken_chars.push(char);
				} else if taken_chars[1] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.push(char);
				} else if taken_chars[2] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.push(char);
				} else if taken_chars[3] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.push(char);
				} else if taken_chars[4] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.push(char);
				} else if taken_chars[5] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.push(char);	
				} else if taken_chars[6] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.push(char);	
				} else if taken_chars[7] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.push(char);	
				} else if taken_chars[7] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.push(char);	
				} else if taken_chars[8] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					// taken_chars.remove(0);
					taken_chars.push(char);	
				} else if taken_chars[9] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					// taken_chars.remove(0);
					taken_chars.push(char);	
				} else if taken_chars[10] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					// taken_chars.remove(0);
					taken_chars.push(char);	
				} else if taken_chars[11] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					// taken_chars.remove(0);
					taken_chars.push(char);	
				} else if taken_chars[12] == char {
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.remove(0);
					taken_chars.push(char);	
				} else {
					taken_chars.clear();
					taken_chars.push(char);
				}
			} else {
				taken_chars.push(char);
			}

			if taken_chars.len() == 14 {
				counter = counter + 1;
				println!("Day 6 part b: {}", counter);
				return counter
			}

			counter = counter + 1;
		}
	}

	println!("Day  part a: {}", total);
	return total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_a_returns_count() {
        let test_input: Vec<String> = vec!["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()];

        let result = solve_a(test_input);
        assert_eq!(result, 11);
    }

	#[test]
    fn solve_a_returns_count_2() {
        let test_input: Vec<String> = vec!["nppdvjthqldpwncqszvftbrmjlhg".to_string()];

        let result = solve_a(test_input);
        assert_eq!(result, 6);
    }

	#[test]
    fn solve_a_returns_count_3() {
        let test_input: Vec<String> = vec!["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()];

        let result = solve_a(test_input);
        assert_eq!(result, 10);
    }

	#[test]
    fn solve_a_returns_count_4() {
        let test_input: Vec<String> = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()];

        let result = solve_a(test_input);
        assert_eq!(result, 5);
    }

    #[test]
    fn solve_b_returns_count() {
        let test_input: Vec<String> = vec!["mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()];

        let result = solve_b(test_input);
        assert_eq!(result, 19);
    }

	#[test]
    fn solve_b_returns_count_2() {
        let test_input: Vec<String> = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()];

        let result = solve_b(test_input);
        assert_eq!(result, 23);
    }

	#[test]
    fn solve_b_returns_count_3() {
        let test_input: Vec<String> = vec!["nppdvjthqldpwncqszvftbrmjlhg".to_string()];

        let result = solve_b(test_input);
        assert_eq!(result, 23);
    }

	#[test]
    fn solve_b_returns_count_4() {
        let test_input: Vec<String> = vec!["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()];

        let result = solve_b(test_input);
        assert_eq!(result, 29);
    }
}
