use crate::helpers::open_file;
use std::collections::HashSet;

use std::io::prelude::*;

pub fn solve() {
	match open_file("day_3") {
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

	match open_file("day_3") {
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
	let mut total: i32 = 0;


    for line in lines {
        let length = line.len();
        let compartments = line.split_at(length / 2);
        let char = find_common_char(compartments.0, compartments.1);
        let score = get_score(char);
        total = total + score
    }

	println!("Day 3 part a: {}", total);
	return total
}

fn find_common_char(a: &str, b: &str) -> char  {
    let set: HashSet<char> = b.chars().collect();

    let char = a.chars().find(|c| set.contains(&c)).unwrap();
    return char
}

fn find_common_char_with_three(a: &String, b: &String, c: &String) -> char  {
    let set: HashSet<char> = b.chars().collect();
    let set2: HashSet<char> = c.chars().collect();

    let char = a.chars().find(|d| set.contains(&d) && set2.contains(&d)).unwrap();
    return char
}

fn get_score(char: char) -> i32 {
    let score;

    match char {
        'a' => score = 1,
        'b' => score = 2,
        'c' => score = 3,
        'd' => score = 4,
        'e' => score = 5,
        'f' => score = 6,
        'g' => score = 7,
        'h' => score = 8,
        'i' => score = 9,
        'j' => score = 10,
        'k' => score = 11,
        'l' => score = 12,
        'm' => score = 13,
        'n' => score = 14,
        'o' => score = 15,
        'p' => score = 16,
        'q' => score = 17,
        'r' => score = 18,
        's' => score = 19,
        't' => score = 20,
        'u' => score = 21,
        'v' => score = 22,
        'w' => score = 23,
        'x' => score = 24,
        'y' => score = 25,
        'z' => score = 26,
        'A' => score = 27,
        'B' => score = 28,
        'C' => score = 29,
        'D' => score = 30,
        'E' => score = 31,
        'F' => score = 32,
        'G' => score = 33,
        'H' => score = 34,
        'I' => score = 35,
        'J' => score = 36,
        'K' => score = 37,
        'L' => score = 38,
        'M' => score = 39,
        'N' => score = 40,
        'O' => score = 41,
        'P' => score = 42,
        'Q' => score = 43,
        'R' => score = 44,
        'S' => score = 45,
        'T' => score = 46,
        'U' => score = 47,
        'V' => score = 48,
        'W' => score = 49,
        'X' => score = 50,
        'Y' => score = 51,
        'Z' => score = 52,
        _ => panic!("I dont know this char")
    }

    return score;
}

fn solve_b(lines: Vec<String>) -> i32  {
	let mut total: i32 = 0;

    let groups: Vec<Vec<String>> = lines.chunks(3).map(|s| s.into()).collect();

    for group in groups {
        let char = find_common_char_with_three(&group[0], &group[1], &group[2]);
        let score = get_score(char);
        
        total = total + score;
    }
	println!("Day 3 part b: {}", total);
	return total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_a_returns_count() {
        let test_input: Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string()
        ];

        let result = solve_a(test_input);
        assert_eq!(result, 157);
    }

    #[test]
    fn solve_b_returns_count() {
        let test_input: Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string()
        ];

        let result = solve_b(test_input);
        assert_eq!(result, 70);
    }
}
