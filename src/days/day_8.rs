use crate::helpers::open_file;
use crate::helpers::gigit;

use std::io::prelude::*;

pub fn solve() {
	match open_file("day_8") {
		Ok(reader) => {
			let mut matrix: Vec<Vec<i32>> = Vec::new();
			for line in reader.lines() {
				if let Ok(val) = line {
					matrix.push(val.chars().filter_map(|x| gigit(x)).collect());
				}
			}
			solve_a(matrix)
		},
		Err(e) => {
			panic!("crash and burn: {}", e);
		}	
	};

	match open_file("day_8") {
		Ok(reader) => {
			let mut matrix: Vec<Vec<i32>> = Vec::new();
			for line in reader.lines() {
				if let Ok(val) = line {
					matrix.push(val.chars().filter_map(|x| gigit(x)).collect());
				}
			}
			solve_b(matrix)
		},
		Err(e) => {
			panic!("crash and burn: {}", e);
		}	
	};
}

fn left(n: usize, line: &Vec<i32>) -> bool {
	let mut visible: bool = false;

	for each in 1..=n {
		if line[n] > line[each - 1] {
			visible = true;
		} else {
			visible = false;
			break;
		}
	}
	
	return visible
}

fn right(n: usize, line: &Vec<i32>) -> bool {
	let mut visible = false;

	for each in n..(line.len() - 1) {
		if line[n] > line[each + 1] {
			visible = true;
		} else {
			visible = false;
			break;
		}
	}

	return visible
}

fn up(n: usize, lines: &Vec<Vec<i32>>, line_number: usize) -> bool {
	let mut visible: bool = false;

	for each in 0..line_number {
		if lines[line_number][n] > lines[each][n] {
			visible = true;
		} else {
			visible = false;
			break;
		}
	}

	return visible
}

fn down(n: usize, lines: &Vec<Vec<i32>>, line_number: usize) -> bool {
	let mut visible: bool = false;
	
	for each in (line_number+1)..lines.len() {
		if lines[line_number][n] > lines[each][n] {
			visible = true;
		} else {
			visible = false;
			break;
		}
	}

	return visible
}

fn solve_a(lines: Vec<Vec<i32>>) -> i32  {
	let mut visible_trees: i32;
	let mut line_number: usize = 0;
	
	let x = *&lines[0].len() as i32;
	let y = *&lines.len() as i32;
	visible_trees = x * 2 + y * 2 - 4;

	
	
	for line in &lines {
		if line_number == 0 {
			line_number = line_number + 1;
			continue;
		}
		if line_number == (y - 1).try_into().unwrap() {
			break;
		}
				
		for n in 1..line.len() - 1 {
			if left(n, &line) {
				visible_trees = visible_trees + 1;
			} else if right(n, &line) {
				visible_trees = visible_trees + 1;
			} else if up(n, &lines, line_number) {
				visible_trees = visible_trees + 1;
			} else if down(n, &lines, line_number) {
				visible_trees = visible_trees + 1;
			}
		}
			
		line_number = line_number + 1;
	}

	println!("Day 8 part a: {}", visible_trees);
	return visible_trees
}

fn look_left(tree_location: usize, tree_line: &Vec<i32>) -> i32 {
	let mut count: i32 = 0;
	
	for location in (1..=tree_location).rev() {
		
		if tree_line[tree_location] > tree_line[location - 1] {
			count = count + 1
		} else {
			count = count + 1;
			break;
		}
	}

	return count
}

fn look_right(tree_location: usize, tree_line: &Vec<i32>) -> i32 {
	let mut count: i32 = 0;

	for each in tree_location..(tree_line.len() - 1) {
		if tree_line[tree_location] > tree_line[each + 1] {
			count = count + 1
		} else {
			count = count + 1;
			break;
		}
	}

	return count
}

fn look_up(tree_location: usize, lines: &Vec<Vec<i32>>, line_number: usize) -> i32 {
	let mut count: i32 = 0;

	for each in (0..line_number).rev() {
		if lines[line_number][tree_location] > lines[each][tree_location] {
			count = count + 1;
		} else {
			count = count + 1;
			break;
		}
	}

	return count
}

fn look_down(tree_location: usize, lines: &Vec<Vec<i32>>, line_number: usize) -> i32 {
	let mut count: i32 = 0;
	
	for each in (line_number+1)..lines.len() {
		if lines[line_number][tree_location] > lines[each][tree_location] {
			count = count + 1;
		} else {
			count = count + 1;
			break;
		}
	}

	return count
}



fn get_score(tree_location: usize, tree_line: &Vec<i32>, lines: &Vec<Vec<i32>>, line_number: usize) -> i32 {
	let left_score = look_left(tree_location, tree_line);
	let right_score = look_right(tree_location, tree_line);
	let up_score = look_up(tree_location, lines, line_number);
	let down_score = look_down(tree_location, lines, line_number);
	
	let total = left_score * right_score * up_score * down_score;

	return total
}

fn solve_b(lines: Vec<Vec<i32>>) -> i32  {
	let mut highest_scenic_score: i32 = 0;
	let mut line_number: i32 = 0;
	let y = *&lines.len() as i32;

	for tree_line in &lines {
		if line_number == 0 {
			line_number = line_number + 1;
			continue;
		}
		if line_number == (y - 1).try_into().unwrap() {
			break;
		}
				
		for tree_location in 1..tree_line.len() - 1 {
			let scenic_score: i32 = get_score(tree_location, tree_line, &lines, line_number.try_into().unwrap());
			if scenic_score > highest_scenic_score {
				highest_scenic_score = scenic_score;
			}

		}
			
		line_number = line_number + 1;
	}


	println!("Day 8 part b: {}", highest_scenic_score);
	return highest_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_a_returns_count() {
		let test_input: Vec<Vec<i32>> = vec![
			vec![3,0,3,7,3], 
			vec![2,5,5,1,2], 
			vec![6,5,3,3,2], 
			vec![3,3,5,4,9], 
			vec![3,5,3,9,0]
		];

        let result = solve_a(test_input);
        assert_eq!(result, 21);
    }

	#[test]
    fn solve_b_returns_count() {
		let test_input: Vec<Vec<i32>> = vec![
			vec![3,0,3,7,3], 
			vec![2,5,5,1,2], 
			vec![6,5,3,3,2], 
			vec![3,3,5,4,9], 
			vec![3,5,3,9,0]
		];

        let result = solve_b(test_input);
        assert_eq!(result, 8);
    }
}
