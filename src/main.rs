pub mod days;
pub mod helpers;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args[1] == "all" {
		days::run_all_days();
	}		
}