
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn compute(path_to_data: &str) -> i32 {

	let mut set: HashSet<i32> = HashSet::new();
	set.insert(0);

	let mut result: i32 = 0;

	loop {

		let f = File::open(path_to_data).unwrap();
		let file = BufReader::new(&f);

		for line in file.lines() {
			let num: i32 = line.unwrap().parse().unwrap();

			result = result + num;

			if !set.contains(&result) {
				set.insert(result);
			}
			else {
				return result;
			}
		}
	}
}
