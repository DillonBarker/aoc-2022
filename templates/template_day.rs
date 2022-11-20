use crate::helpers::get_input_in_lines;

const DAY: &str = "day_";

pub fn solve() {
	let input = get_input_in_lines(DAY);
	solve_a(input.clone());
	solve_b(input);
}

fn solve_a(input: String) -> i32 {
	let mut count: i32 = 0;

	println!("{} part A answer is: {}", DAY, count);
	return count
}

fn solve_b(input: String) -> i32 {
	let mut count: i32 = 0;

	println!("{} part B answer is: {}", DAY, count);
	return count
}

#[cfg(test)]
mod tests {
    use super::solve_a;
		use super::solve_b;

		const INPUT: &str = "";

		#[test]
    fn solve_a_returns_count() {
			let result = solve_a(INPUT.to_string());
			assert_eq!(result, 7);
		}

		#[test]
    fn solve_b_returns_count() {
			let result = solve_b(INPUT.to_string());
			assert_eq!(result, 5);
		}
}
