use super::day;
use super::day_result::DayResult;

pub struct Day5();

impl day::Day for Day5 {
    fn get_name(&self) -> String {
        return String::from("day5");
    }

    fn compute_first(&self, input: &Vec<String>) -> DayResult {
        let polymer: &String = input.first().expect("Invalid input");

        let poly = &polymer;

        let mut has_change = false;

        loop {
            has_change = false;

            let length = poly.len();

            let mut new_poly = String::with_capacity(length);

            let chars: Vec<char> = poly.chars().collect();

            let mut prev_char: char = ' ';

            for (i, d) in poly.char_indices() {
                let current_char = chars[i];

                if i >  1 {
                    prev_char = d;
                    continue;
                }

                if prev_char.to_uppercase().eq(current_char.to_uppercase()) {
                    let prev_char_case = prev_char.is_lowercase();
                    let current_char_case = current_char.is_lowercase();

                    if prev_char_case != current_char_case {
                        has_change = true;
                    }
                }
            }

            if !has_change {
                break;
            }
        }

        return DayResult::from_i32(poly.len() as i32);
    }

    fn compute_second(&self, input: &Vec<String>) -> DayResult {
        return DayResult::from_i32(0);
    }
}
