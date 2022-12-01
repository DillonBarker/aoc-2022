pub mod days;
pub mod helpers;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args[1] == "all" {
		days::run_all_days();
	}
	
	// TODO: find a nicer way of doing the below
	if args[1] == "0" {
		days::day_0::solve();
	}

	if args[1] == "0.5" {
		days::day_0_5::solve();
	}
	
	if args[1] == "1" {
		days::day_1::solve();
	}
}