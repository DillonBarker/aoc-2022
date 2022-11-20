use std::fs;

pub fn get_input_in_lines(day: &str) -> String {
	let mut file_path: String = "inputs/".to_owned();
	let txt: &str = ".txt";
	file_path.push_str(day);
	file_path.push_str(&txt);
	
	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read file.");

	return contents
}