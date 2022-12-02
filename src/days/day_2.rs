use crate::helpers::open_file;

use std::io::prelude::*;

pub fn solve() {
	match open_file("day_2") {
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

	match open_file("day_2") {
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

struct Columns<'a> {
    first: &'a str,
    second: &'a str
}

const WIN: i32 = 6;
const DRAW: i32 = 3;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn solve_a(lines: Vec<String>) -> i32  {
    let mut score: i32 = 0;

    for line in lines {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        let columns = Columns {
            first: line_split[0],
            second: line_split[1]
        };

        let theirs;
        let mine;

        match columns.first {
            "A" => theirs = "rock",
            "B" => theirs = "paper",
            "C" => theirs = "scissors",
            _ => panic!("I dont know that letter.")
        };

        match columns.second {
            "X" => mine = "rock",
            "Y" => mine = "paper",
            "Z" => mine = "scissors",
            _ => panic!("I dont know that letter.")
        };

        if theirs == "rock" && mine == "paper" {
            score = score + WIN + PAPER
        }
        if theirs == "paper" && mine == "scissors" {
            score = score + WIN + SCISSORS
        }
        if theirs == "scissors" && mine == "rock" {
            score = score + WIN + ROCK
        }

        if theirs == mine {
            let hand;
            match theirs {
                "rock" => hand = ROCK,
                "paper" => hand = PAPER,
                "scissors" => hand = SCISSORS,
                _ => panic!("I dont know that hand.")
            }
            score = score + DRAW + hand
        }

        if theirs == "rock" && mine == "scissors" {
            score = score + SCISSORS
        }
        if theirs == "paper" && mine == "rock" {
            score = score + ROCK
        }
        if theirs == "scissors" && mine == "paper" {
            score = score + PAPER
        }
    }

	println!("Day 1 part a: {}", score);
	return score
}

fn solve_b(lines: Vec<String>) -> i32  {
    let mut score: i32 = 0;

    for line in lines {
        let line_split: Vec<&str> = line.split_whitespace().collect();

        let columns = Columns {
            first: line_split[0],
            second: line_split[1]
        };

        if columns.second == "X" {
            // i need to lose
            if columns.first == "A" {
                score = score + 3
            }
            if columns.first == "B" {
                score = score + 1
            }
            if columns.first == "C" {
                score = score + 2
            }
        }
        if columns.second == "Y" {
            // i need to draw
            if columns.first == "A" {
                score = score + 4
            }
            if columns.first == "B" {
                score = score + 5
            }
            if columns.first == "C" {
                score = score + 6
            }
        }
        if columns.second == "Z" {
            // i need to win
            if columns.first == "A" {
                score = score + 6 + 2
            }
            if columns.first == "B" {
                score = score + 6 + 3
            }
            if columns.first == "C" {
                score = score + 6 + 1
            }
        }
    }

	println!("Day 1 part b: {}", score);
	return score
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn solve_a_returns_count() {
//         // let test_input: Vec<String> = vec![A Y
//         // B X
//         // C Z];

//         let result = solve_a(test_input);
//         assert_eq!(result, 24000);
//     }

//     #[test]
//     fn solve_b_returns_count() {
//         let test_input: Vec<String> = vec!["1000".to_string(), "2000".to_string(), 
//             "3000".to_string(), "".to_string(), "4000".to_string(), "".to_string(), 
//             "5000".to_string(), "6000".to_string(), "".to_string(), "7000".to_string(), 
//             "8000".to_string(), "9000".to_string(), "".to_string(), "10000".to_string()];

//         let result = solve_b(test_input);
//         assert_eq!(result, 41000);
//     }
// }
