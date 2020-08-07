use super::day;
use super::day_result::DayResult;
use std::cell::RefCell;

pub struct Day5();

impl day::Day for Day5 {
    fn get_name(&self) -> String {
        return String::from("day5");
    }

    fn compute_first(&self, input: &Vec<String>) -> DayResult {
        let polymer: &String = input.first().expect("Invalid input");

        println!("polymer {}", polymer);

        let mut poly = RefCell::new(polymer.to_string());

        let mut has_change = false;

        let mut new_poly: RefCell<String>;

        loop {
            has_change = false;

            let length = poly.borrow().len();

            new_poly = RefCell::new(String::with_capacity(length));

            let chars: Vec<char> = poly.borrow().chars().collect();

            let mut prev_char: char = ' ';

            for (i, d) in poly.borrow().char_indices() {
                let current_char = chars[i];

                println!("current_char {}", current_char);

                if i < 1 {
                    prev_char = d;
                    continue;
                }

                if prev_char.to_uppercase().eq(current_char.to_uppercase()) {
                    let prev_char_case = prev_char.is_lowercase();
                    let current_char_case = current_char.is_lowercase();

                    if prev_char_case == current_char_case {
                        new_poly.borrow_mut().push(current_char);
                        println!("added");
                    }
                } else {
                    new_poly.borrow_mut().push(current_char);
                    println!("added");
                }
            }

            if new_poly.borrow().len() == poly.borrow().len() {
                break;
            }

            poly = new_poly;
        }

        return DayResult::from_i32(poly.borrow().len() as i32);
    }

    fn compute_second(&self, input: &Vec<String>) -> DayResult {
        return DayResult::from_i32(0);
    }
}
