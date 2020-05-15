use super::day;
use super::day_result::DayResult;
use std::collections::HashSet;

pub struct Day1();

impl day::Day for Day1 {
    fn get_name(&self) -> String {
        return String::from("day1");
    }

    fn compute_first(&self, input: &Vec<String>) -> DayResult {
        let mut result: i32 = 0;

        for line in input {
            let num: i32 = match line.parse() {
                Ok(num) => num,
                Err(_) => panic!("Unable to parse line"),
            };
            result = result + num;
        }
        return DayResult::from_i32(result);
    }

    fn compute_second(&self, input: &Vec<String>) -> DayResult {
        let mut set: HashSet<i32> = HashSet::new();
        set.insert(0);
        let mut result: i32 = 0;
        loop {
            for line in input {
                let num: i32 = match line.parse() {
                    Ok(num) => num,
                    Err(_) => panic!("Unable to parse number"),
                };
                result = result + num;
                if !set.contains(&result) {
                    set.insert(result);
                } else {
                    return DayResult::from_i32(result);
                }
            }
        }
    }
}
