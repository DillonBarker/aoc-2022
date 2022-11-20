use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_input_as_string(day: &str) -> String {
	let mut file_path: String = "inputs/".to_owned();
	let txt: &str = ".txt";
	file_path.push_str(day);
	file_path.push_str(&txt);
	
	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read file.");

	return contents
}

pub fn get_input_as_lines(day: &str) -> Result<std::io::Lines<io::BufReader<fs::File>>, std::io::Error> {
	let mut file_path: String = "inputs/".to_owned();
	let txt: &str = ".txt";
	file_path.push_str(day);
	file_path.push_str(&txt);
	
	return read_lines(file_path)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}