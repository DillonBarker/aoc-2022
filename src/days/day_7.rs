use crate::helpers::open_file;

use std::io::prelude::*;

pub fn solve() {
	match open_file("day_7") {
		Ok(reader) => {
			// let lines: Vec<u8> = reader.split(b'$').map(|item| item.unwrap()).collect()
			// let lines: Vec<String> = reader.lines().
			// 	map(|item| item.unwrap()).
			// 	collect();
			// solve_a(lines);
		},
		Err(e) => {
			panic!("crash and burn: {}", e);
		}
	};

	// match open_file("day_7") {
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

#[derive(Debug, Clone)]
struct Dir {
	name: String,
	size: i32,
	children: Vec<Dir>
}

fn read_line_into_dir(line:  String) -> Dir {
	let split_line: Vec<&str> = line.split(" ").collect();

	return Dir {
		name: split_line[1].to_string(),
		size: 0,
		children: Vec::new()
	}
}

fn read_line_into_file_size(line: String) -> i32 {
	let split_line: Vec<&str> = line.split(" ").collect();
	let file_size = split_line[0].parse().unwrap();

	return file_size
}

fn read_cmd_line_into_dir(line:  String) -> Dir {
	let split_line: Vec<&str> = line.split(" ").collect();

	return Dir {
		name: split_line[2].to_string(),
		size: 0,
		children: Vec::new()
	}
}

fn solve_a(lines: Vec<String>) -> i32  {
	let mut dirs: Vec<Dir> = Vec::new();
	let mut current_dir: Dir = Dir {
		name: "/".to_string(),
		size: 0,
		children: Vec::new()
	};

	dirs.push(current_dir);
	let thing = "string";
	
	while thing != "$ ls" {
		
	}

	dbg!(dirs);

	println!("Day 1 part a: {}", 0);
	return 0
}

// fn solve_b(lines: Vec<String>) -> i32  {
// 	let mut total_cal: i32 = 0;

// 	println!("Day 1 part b: {}", total_cal);
// 	return total_cal;
// }

#[cfg(test)]
mod tests {
    use super::*;
	
    #[test]
    fn solve_a_returns_count() {
        let test_input: Vec<String> = vec!["$ cd /".to_string(), "$ ls".to_string(), 
            "dir a".to_string(), "14848514 b.txt".to_string(), "8504156 c.dat".to_string(), "dir d".to_string(), 
            "$ cd a".to_string(), "$ ls".to_string(), "dir e".to_string(), "29116 f".to_string(), 
            "2557 g".to_string(), "62596 h.lst".to_string(), "$ cd e".to_string(), "$ ls".to_string(), 
            "584 i".to_string(), "$ cd ..".to_string(), "$ cd ..".to_string(), "$ cd d".to_string(), 
            "$ ls".to_string(), "4060174 j".to_string(), "8033020 d.log".to_string(), "5626152 d.ext".to_string(), "7214296 k".to_string()];

        let result = solve_a(test_input);
        assert_eq!(result, 24000);
    }

    // #[test]
    // fn solve_b_returns_count() {
    //     let test_input: Vec<String> = vec!["1000".to_string(), "2000".to_string(), 
    //         "3000".to_string(), "".to_string(), "4000".to_string(), "".to_string(), 
    //         "5000".to_string(), "6000".to_string(), "".to_string(), "7000".to_string(), 
    //         "8000".to_string(), "9000".to_string(), "".to_string(), "10000".to_string()];

    //     let result = solve_b(test_input);
    //     assert_eq!(result, 41000);
    // }
}
