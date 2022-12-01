use std::fs::{self, File};
use std::io::{BufReader, Error};

pub fn get_input_as_string(day: &str) -> String {
	let mut file_path: String = "inputs/".to_owned();
	let txt: &str = ".txt";
	file_path.push_str(day);
	file_path.push_str(&txt);
	
	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read file.");

	return contents
}

pub fn open_file(day: &str) -> Result<BufReader<File>, Error> {
	let mut file_path: String = "inputs/".to_owned();
	let txt: &str = ".txt";
	file_path.push_str(day);
	file_path.push_str(&txt);		

	let file = File::open("inputs/day_1.txt")?;
	return Ok(BufReader::new(file));
}
