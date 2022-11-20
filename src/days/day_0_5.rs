use crate::helpers::helpers::get_input_in_lines;

const DAY: &str = "day_0_5";

pub fn solve() {
	let input = get_input_in_lines(DAY);
	solve_a(input.clone());
	solve_b(input);
}

fn solve_a(input: String) -> i32 {
	let mut depth: i32 = 0;
	let mut hoz: i32 = 0;

	let input_as_string: Vec<String> = input
		.split_whitespace()
		.map(|s| s.parse().expect("parse error"))
		.collect();

	for (i, x) in input_as_string.iter().enumerate() {
		if x == "forward" {
			hoz = hoz + input_as_string[i + 1].parse::<i32>().unwrap();
		};

		if x == "up" {
			depth = depth - input_as_string[i + 1].parse::<i32>().unwrap();
		};

		if x == "down" {
			depth = depth + input_as_string[i + 1].parse::<i32>().unwrap();
		};
	};
		
	let ans = hoz * depth;

	println!("{} part A answer is: {}", DAY, ans);
	return ans
}

fn solve_b(input: String) -> i32 {
	let mut depth: i32 = 0;
	let mut hoz: i32 = 0;
	let mut aim: i32 = 0;

	let input_as_string: Vec<String> = input
		.split_whitespace()
		.map(|s| s.parse().expect("parse error"))
		.collect();

	for (i, x) in input_as_string.iter().enumerate() {
		if x == "forward" {
			hoz = hoz + input_as_string[i + 1].parse::<i32>().unwrap();
			depth = depth + (input_as_string[i + 1].parse::<i32>().unwrap() * aim);
		};

		if x == "up" {
			aim = aim - input_as_string[i + 1].parse::<i32>().unwrap();
		};

		if x == "down" {
			aim = aim + input_as_string[i + 1].parse::<i32>().unwrap();
		};
	};
		
	let ans = hoz * depth;

	println!("{} part B answer is: {}", DAY, ans);
	return ans
}

#[cfg(test)]
mod tests {
    use super::solve_a;
		use super::solve_b;

		const INPUT: &str = "forward 5
		down 5
		forward 8
		up 3
		down 8
		forward 2";

		#[test]
    fn solve_a_returns_count() {
			let result = solve_a(INPUT.to_string());
			assert_eq!(result, 150);
		}

		#[test]
    fn solve_b_returns_count() {
			let result = solve_b(INPUT.to_string());
			assert_eq!(result, 900);
		}
}
