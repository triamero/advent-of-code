use std::collections::HashSet;

pub fn compute(input: Vec<String>) -> i32 {

	let mut set: HashSet<i32> = HashSet::new();
	set.insert(0);

	let mut result: i32 = 0;

	loop {

		for line in &input {

			let num: i32 = match line.parse() {
				Ok(num) => num,
				Err(_) => panic!("Unable to parse number")
			};

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
