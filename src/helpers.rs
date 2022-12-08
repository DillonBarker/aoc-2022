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

	let file = File::open(file_path)?;
	return Ok(BufReader::new(file));
}

pub fn gigit(c: char) -> Option<i32> {
    if c as u32 >= '0' as u32 && c as u32 <= '9' as u32 {
        Some((c as u32 - '0' as u32).try_into().unwrap())
    } else {
        None
    }
}
