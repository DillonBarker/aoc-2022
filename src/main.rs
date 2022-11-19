mod day_0;
mod day_0_5;

use std::env;
use std::fs; 

const DAY_0: &str = "day_0";
const DAY_0_5: &str = "day_0_5";

fn main() {
	// You have to input something like `cargo run day_0`

	let mut file_path: String = "inputs/".to_owned();
	let args: Vec<String> = env::args().collect();
	let txt: &str = ".txt";

	file_path.push_str(&args[1]);
	file_path.push_str(&txt);

	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read file");
	
		if args[1] == DAY_0 {
			day_0::solve(contents.clone());
		}
		if args[1] == DAY_0_5 {
			day_0_5::solve(contents);
		}
}