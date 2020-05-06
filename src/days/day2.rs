use super::day;
use std::collections::HashMap;

pub struct Day2();

impl day::Day for Day2 {
    fn get_name(&self) -> String {
        return String::from("day2");
    }

    fn compute_first(&self, input: &Vec<String>) -> i32 {
        let mut result: (i32, i32) = (0, 0);
        for line in input {
            let ln: String = match line.parse() {
                Ok(ln) => ln,
                Err(_) => panic!("Invalid string"),
            };

            let line_result: (i32, i32) = first_process_line(ln);
            result.0 = result.0 + line_result.0;
            result.1 = result.1 + line_result.1;
        }
        return result.0 * result.1;
    }

    fn compute_second(&self, input: &Vec<String>) -> i32 {
        let mut result: (i32, i32) = (0, 0);
        for line in input {

            let ln: String = match line.parse() {
                Ok(ln) => ln,
                Err(_) => panic!("Invalid string"),
            };

            let line_result: (i32, i32) = second_process_line(ln);
            result.0 = result.0 + line_result.0;
            result.1 = result.1 + line_result.1;
        }
        return result.0 * result.1;
    }
}

fn first_process_line(line: String) -> (i32, i32) {
    let mut map: HashMap<char, i32> = HashMap::new();

    for c in line.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    let has_two: bool = map.values().filter(|x: &&i32| x == &&2i32).count() > 0;
    let has_three: bool = map.values().filter(|x| x == &&3i32).count() > 0;

    if has_two && has_three {
        return (1, 1);
    } else if has_two {
        return (1, 0);
    } else if has_three {
        return (0, 1);
    }

    return (0, 0);
}

fn second_process_line(line: String) -> (i32, i32) {
    let mut map: HashMap<char, i32> = HashMap::new();

    for c in line.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    let has_two: bool = map.values().filter(|x: &&i32| x == &&2i32).count() > 0;
    let has_three: bool = map.values().filter(|x| x == &&3i32).count() > 0;

    if has_two && has_three {
        return (1, 1);
    } else if has_two {
        return (1, 0);
    } else if has_three {
        return (0, 1);
    }

    return (0, 0);
}
