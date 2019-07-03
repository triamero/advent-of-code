pub fn compute(input: Vec<String>) -> i32 {

	let mut result: i32 = 0;

	for line in input {
		let num:i32 = match line.parse()  {
			Ok(num) => num,
			Err(_) => panic!("Unable to parse line")
		};

		result = result + num;
	}

	return result;
}