use crate::helpers::helpers::get_input_as_string;

const DAY: &str = "day_0";

pub fn solve() {
	let input = get_input_as_string(DAY);
	solve_a(input.clone());
	solve_b(input);
}

fn solve_a(input: String) -> i32 {
	let input_as_vec: Vec<i32> = input
		.split_whitespace()
		.map(|s| s.parse().expect("parse error"))
		.collect();

	let mut count: i32 = 0;

	for (i, x) in input_as_vec.iter().enumerate() {
		if i != 0 && x > &input_as_vec[i - 1] {
			count = count + 1
		}
	};

	println!("{} part A answer is: {}", DAY, count);
	return count
}

fn solve_b(input: String) -> i32 {
	let input_as_vec: Vec<i32> = input
		.split_whitespace()
		.map(|s| s.parse().expect("parse error"))
		.collect();

	let mut count: i32 = 0;

	for (i, x) in input_as_vec.iter().enumerate() {
		if i != 0 {

			if i + 2 == input_as_vec.len() {
				break;
			}


			if (x + &input_as_vec[i + 1] + &input_as_vec[i + 2]) >  (&input_as_vec[i - 1] + &input_as_vec[i] + &input_as_vec[i + 1]) {
				count = count + 1
			}
		}
	};

	println!("{} part B answer is: {}", DAY, count);
	return count
}

#[cfg(test)]
mod tests {
    use super::solve_a;
		use super::solve_b;

		const INPUT: &str = "199 200 208 210 200 207 240 269 260 263";

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
