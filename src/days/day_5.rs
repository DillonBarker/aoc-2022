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

	match open_file("day_5") {
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

#[derive(Debug, Clone)]
struct Move {
    amount: usize,
    from: usize,
    to: usize
}

fn parse_line(line: String) -> Move {
    let nums: Vec<char> = line.chars().filter(|char| char.is_numeric()).collect();
    let x;
    let x1;
    let x2;
    let y;
    let z;

    if nums.len() == 4 {
        x1 = usize::try_from(nums[0].to_digit(10).unwrap()).unwrap();
        x2 = usize::try_from(nums[1].to_digit(10).unwrap()).unwrap();
        x = (x1.to_string() + &x2.to_string()).parse::<usize>().unwrap();
        y = usize::try_from(nums[2].to_digit(10).unwrap()).unwrap();
        z = usize::try_from(nums[3].to_digit(10).unwrap()).unwrap();
    } else {
        x = usize::try_from(nums[0].to_digit(10).unwrap()).unwrap();
        y = usize::try_from(nums[1].to_digit(10).unwrap()).unwrap();
        z = usize::try_from(nums[2].to_digit(10).unwrap()).unwrap();
    }

    

    return Move {
        amount: x,
        from: y,
        to: z
    }
}

impl<T> VecExt<T> for Vec<T> {}
fn move_containers(movement: Move, mut stacks: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut containers: Vec<String> = Vec::new();
    let mut x: usize = 0;
    while x < movement.amount {
        if stacks[(movement.from - 1)].is_empty() { break };
        let container = stacks[(movement.from - 1)].remove(0);
        containers.insert_from_slice(0 ,&[container]);
        x = x + 1;
    }
    for container in containers.iter().rev() {
        stacks[(movement.to - 1)].insert_from_slice(0, &[container.to_string()]);
    }
    return stacks;
}

fn move_containers_b(movement: Move, mut stacks: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut containers: Vec<String> = Vec::new();
    let mut x: usize = 0;
    while x < movement.amount {
        if stacks[(movement.from - 1)].is_empty() { break };
        let container = stacks[(movement.from - 1)].remove(0);
        containers.push(container);
        x = x + 1;
    }
    for container in containers.iter().rev() {
        stacks[(movement.to - 1)].insert_from_slice(0, &[container.to_string()]);
    }
    return stacks;
}


pub trait VecExt<T>: AsMut<Vec<T>> {
    fn insert_from_slice(&mut self, index: usize, other: &[T])
    where
        T: Clone,
    {
        self.as_mut()
            .splice(index..index, other.into_iter().cloned());
    }
}


fn solve_a(lines: Vec<String>) -> String  {
    let mut stacks: Vec<Vec<String>> = Vec::new();

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
            let mut counter: usize = 0;

            for each in thing {
                let mut stack: Vec<String> = Vec::new();

                if each == "x" {
                    if stacks.get_mut(counter).is_none() {
                        stacks.push(stack);
                    }

                } else {
                    if stacks.get_mut(counter).is_none() {
                        stack.push(each.to_string());
                        stacks.push(stack);
                    } else {
                        stacks[counter].push(each.to_string());
                    }
                }
                counter = counter + 1;
            }
        } else if !line.contains('[') && !line.contains("move") {
            // this is the numbering of the crates we might be able to skip this
        } else if line.is_empty() {
            // this is just a line break, i could split here
        } else if line.contains("move") {
            let movement: Move = parse_line(line.clone());
            stacks = move_containers(movement, stacks);
        }
    }

    let mut str: String = "".to_owned();
    for stack in stacks {
        str.push_str(&stack[0])
    }

	println!("Day 5 part a: {}", str);
	return str
}

fn solve_b(lines: Vec<String>) -> String  {
    let mut stacks: Vec<Vec<String>> = Vec::new();

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
            let mut counter: usize = 0;

            for each in thing {
                let mut stack: Vec<String> = Vec::new();

                if each == "x" {
                    if stacks.get_mut(counter).is_none() {
                        stacks.push(stack);
                    }

                } else {
                    if stacks.get_mut(counter).is_none() {
                        stack.push(each.to_string());
                        stacks.push(stack);
                    } else {
                        stacks[counter].push(each.to_string());
                    }
                }
                counter = counter + 1;
            }
        } else if line.contains("move") {
            let movement: Move = parse_line(line.clone());
            stacks = move_containers_b(movement, stacks);
        }
    }

    let mut str: String = "".to_owned();
    for stack in stacks {
        str.push_str(&stack[0])
    }

	println!("Day 5 part b: {}", str);
	return str
}


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

    #[test]
    fn solve_b_returns_count() {
        let test_input: Vec<String> = vec![
            "    [D]    ".to_string(), "[N] [C]    ".to_string(), 
            "[Z] [M] [P]".to_string(), " 1   2   3 ".to_string(), "           ".to_string(),
            "move 1 from 2 to 1".to_string(), "move 3 from 1 to 3".to_string(),
             "move 2 from 2 to 1".to_string(), "move 1 from 1 to 2".to_string()];

        let result = solve_b(test_input);
        assert_eq!(result, "MCD");
    }
}
