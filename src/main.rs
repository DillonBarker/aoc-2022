pub mod days;
pub mod helpers;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args[1] == "all" {
		days::run_all_days();
	}
	
	// TODO: find a nicer way of doing the below
	if args[1] == "1" {
		days::day_1::solve();
	}

    if args[1] == "2" {
		days::day_2::solve();
	}

    if args[1] == "3" {
        days::day_3::solve();
    }

    if args[1] == "4" {
        days::day_4::solve();
    }

    if args[1] == "5" {
        days::day_5::solve();
    }

    if args[1] == "6" {
        days::day_6::solve();
    }
}