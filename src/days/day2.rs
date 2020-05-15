use super::day;
use super::day_result::DayResult;
use std::collections::HashMap;

pub struct Day2();

/// Система инвентаризации
impl day::Day for Day2 {
    fn get_name(&self) -> String {
        return String::from("day2");
    }

    fn compute_first(&self, input: &Vec<String>) -> DayResult {
        let mut result: (i32, i32) = (0, 0);
        for line in input {
            let ln: String = match line.parse() {
                Ok(ln) => ln,
                Err(_) => panic!("Invalid string"),
            };

            let line_result: (i32, i32) = first_process_line(ln);
            result.0 += line_result.0;
            result.1 += line_result.1;
        }

        return DayResult::from_i32(result.0 * result.1);
    }

    fn compute_second(&self, input: &Vec<String>) -> DayResult {
        let mut result: (usize, usize, i32);

        let first_lines_matching = count_mismatchings(&input[0], &input[1]);

        let mut str_result = first_lines_matching.0;
        result = (0, 1, first_lines_matching.1);
        

        for (i, x) in input.iter().enumerate() {
            for (j, y) in input.iter().enumerate() {
                if i == j {
                    continue;
                }

                let cnt = count_mismatchings(x, y);

                if cnt.1 < result.2 {
                    result.0 = i;
                    result.1 = j;
                    result.2 = cnt.1;
                    str_result = cnt.0;
                }
            }
        }

        return DayResult::from_string(str_result);
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

fn count_mismatchings(str1: &str, str2: &str) -> (String, i32) {
    let mut count = 0;
    let mut str_result = "".to_string();

    let str1 = str1.as_bytes();
    let str2 = str2.as_bytes();

    for i in 0..str1.len() {
        if str1[i] != str2[i] {
            count += 1;
        } else {
            str_result.push(str1[i] as char);
        }
    }

    (str_result.to_string(), count)
}
