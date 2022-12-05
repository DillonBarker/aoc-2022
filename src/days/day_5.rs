use std::{io::{BufRead}};

use crate::helpers::open_file;

pub fn solve() {
	match open_file("day_5") {
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

	// match open_file("day_5") {
	// 	Ok(reader) => {
	// 		let lines: Vec<String> = reader.lines().
	// 			map(|item| item.unwrap()).
	// 			collect();
	// 		solve_b(lines);
	// 	},
	// 	Err(e) => {
	// 		panic!("crash and burn: {}", e);
	// 	}
	// };
}

#[derive(Clone)]
struct Stack(i32, Vec<String>);

#[derive(Clone)]
struct Stacks<'a>(Vec<&'a Stack>);

fn solve_a(lines: Vec<String>) -> &'static str  {
    let st: Vec<&Stack> = Vec::new();
    let mut stacks = Stacks(st);

	for line in lines {
        if line.contains('[') {
            // FUCK THIS ALL
            let new_line = line.replace('[', "");
            let newer_line = new_line.replace(']', "");
            let mut test = newer_line.replace("    ", "x");
            let cleaned_string: String = remove_whitespace(&mut test);
            let mut thing: Vec<&str> = cleaned_string.split("").collect();
            thing.pop();
            thing.remove(0);
            // FUCK THIS ALL
            
            dbg!(thing);
            let mut counter: i32 = 1;
            for each in thing {
                // let mut new_stack: &Stack;

                if each == "x" {
                    // initialise empty stack
                    let vec: Vec<String> = Vec::new();
                    let new_stack = &Stack(counter, vec);
                } else {
                    for stack in &stacks.0 {
                        if stack.0 != counter {
                            let mut vec: Vec<String> = Vec::new();
                            vec.push(each.to_string());
                            let new_stack = &Stack(counter, vec);  
                        } else {
                            let new_stack = stack;
                            new_stack.1.push(each.to_string());
                        }
                        
                    }
                }
                // stacks.0.push(new_stack);

                counter = counter + 1;
            }


        } else if !line.contains('[') && !line.contains("move") {
            // this is the numbering of the crates we might be able to skip this
        } else if line.is_empty() {
            // this is just a line break, i could split here
        } else if line.contains("move") {
            // movements
        }
    }

	println!("Day 1 part a: {}", "str");
	return "str"
}

// fn solve_b(lines: Vec<String>) -> i32  {
// 	let mut total_cal: i32 = 0;

// 	println!("Day 1 part b: {}", total_cal);
// 	return total_cal;
// }

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_a_returns_count() {
        let test_input: Vec<String> = vec![
            "    [D]    ".to_string(), "[N] [C]    ".to_string(), 
            "[Z] [M] [P]".to_string(), " 1   2   3 ".to_string(), "           ".to_string(),
            "move 1 from 2 to 1".to_string(), "move 3 from 1 to 3".to_string(),
             "move 2 from 2 to 1".to_string(), "move 1 from 1 to 2".to_string()];

        let result = solve_a(test_input);
        assert_eq!(result, "CMZ");
    }

    // #[test]
    // fn solve_b_returns_count() {
    //     let test_input: Vec<String> = vec!["    [D]    ".to_string(), "2000".to_string(), 
    //         "3000".to_string(), "".to_string(), "4000".to_string(), "".to_string(), 
    //         "5000".to_string(), "6000".to_string(), "".to_string(), "7000".to_string(), 
    //         "8000".to_string(), "9000".to_string(), "".to_string(), "10000".to_string()];

    //     let result = solve_b(test_input);
    //     assert_eq!(result, 41000);
    // }
}
