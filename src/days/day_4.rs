use crate::helpers::open_file;

use std::io::prelude::*;

pub fn solve() {
	match open_file("day_4") {
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

	match open_file("day_4") {
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
    let mut score: i32 = 0;

    for line in lines {
        let pair: Vec<&str> = line.split(",").collect();
        let elf1: Vec<i32> = pair[0].split("-").map(|s| s.parse::<i32>().unwrap()).collect();
        let elf2: Vec<i32> = pair[1].split("-").map(|s| s.parse::<i32>().unwrap()).collect();
        let elf1 = elf1[0]..=elf1[1];
        let elf2 = elf2[0]..=elf2[1];
        let mut yes: bool = false;

        if elf1.size_hint() > elf2.size_hint() {
            for each in elf2 {
                yes = elf1.contains(&each);
                if !yes { break };      
            }
            if yes {
                score = score + 1;
            }
        } else if elf2.size_hint() > elf1.size_hint() {
            for each in elf1 {
                yes = elf2.contains(&each);
                if !yes { break };      
            }
            if yes {
                score = score + 1;
            }
        } else {
            for each in elf1 {
                yes = elf2.contains(&each);
                if !yes { break };      
            }
            if yes {
                score = score + 1;
            }
        }
    }
	println!("Day 4 part a: {}", score);
	return score
}

fn solve_b(lines: Vec<String>) -> i32  {
	let mut score: i32 = 0;

    for line in lines {
        let pair: Vec<&str> = line.split(",").collect();
        let elf1: Vec<i32> = pair[0].split("-").map(|s| s.parse::<i32>().unwrap()).collect();
        let elf2: Vec<i32> = pair[1].split("-").map(|s| s.parse::<i32>().unwrap()).collect();
        let elf1 = elf1[0]..=elf1[1];
        let elf2 = elf2[0]..=elf2[1];
        let mut yes: bool;

        for each in elf2 {
            yes = elf1.contains(&each);
            if yes {
                score = score + 1;
                break;
            }
        }
    }
	println!("Day 4 part b: {}", score);
	return score
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn solve_a_returns_count() {
        let test_input: Vec<String> = vec![
            "2-4,6-8".to_string(), "2-3,4-5".to_string(), 
            "5-7,7-9".to_string(), "2-8,3-7".to_string(), 
            "6-6,4-6".to_string(), "2-6,4-8".to_string(), 
        ];

        let result = solve_a(test_input);
        assert_eq!(result, 2);
    }

    #[test]
    fn solve_b_returns_count() {
        let test_input: Vec<String> = vec![
            "2-4,6-8".to_string(), "2-3,4-5".to_string(), 
            "5-7,7-9".to_string(), "2-8,3-7".to_string(), 
            "6-6,4-6".to_string(), "2-6,4-8".to_string(), 
        ];

        let result = solve_b(test_input);
        assert_eq!(result, 4);
    }
}
