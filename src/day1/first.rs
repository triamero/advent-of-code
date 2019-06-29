use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn compute(path_to_data: &str) -> i32 {
	let f = File::open(path_to_data).unwrap();
	let file = BufReader::new(&f);

	let mut result: i32 = 0;

	for line in file.lines() {
		let num : i32 = line.unwrap().parse().unwrap();

		result = result + num;
	}

	return result;
}