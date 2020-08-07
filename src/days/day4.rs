extern crate chrono;

use super::day;
use super::day_result::DayResult;

use chrono::NaiveDateTime;

pub struct Day4();

impl day::Day for Day4 {
    fn get_name(&self) -> String {
        return String::from("day4");
    }

    fn compute_first(&self, input: &Vec<String>) -> DayResult {
        let mut logs = parse_lines(input);

        logs.sort_by(|x, y| x.timestamp.cmp(&y.timestamp));

        let mut infos: Vec<(i32, Vec<bool>)> = Vec::new();

        let mut guard_id = -1;

        for ln in logs.iter() {
            // println!("[{}] {}", ln.timestamp, ln.data);

            match ln.data.chars().next() {
                Some('G') => {
                    // [0]=Guard [1]=#xxx [2]=begins [3]=shift
                    let splits: Vec<&str> = ln.data.split(' ').collect();

                    guard_id = splits[1].trim_matches('#').parse().unwrap();

                    println!("New guard id {}", guard_id);
                    
                },
                Some('f') => println!("Guard fall asleep"),
                Some('w') => println!("Guard wakes up"),
                Some(_) => panic!("Unknown action"),
                None => panic!("Unknown action"),
            }
        }

        return DayResult::from_i32(-1);
    }

    fn compute_second(&self, input: &Vec<String>) -> DayResult {
        return DayResult::from_i32(-1);
    }
}

struct Guard {
    id: i32,
}

struct GuardAction {
    timestamp: NaiveDateTime,
    sleeping: bool,
    sheeping: bool,
}

struct LogLine {
    timestamp: NaiveDateTime,
    data: String,
}

impl LogLine {
    fn new(ts: NaiveDateTime, data: String) -> LogLine {
        LogLine {
            timestamp: ts,
            data: data,
        }
    }
}

fn parse_lines(input: &Vec<String>) -> Vec<LogLine> {
    let mut result: Vec<LogLine> = Vec::new();

    for line in input.iter() {
        let splits: Vec<&str> = line.split(']').collect();

        let ts_str = splits[0].trim().trim_matches('[');

        // println!("'{}'", ts_str);

        let timestamp = NaiveDateTime::parse_from_str(ts_str, "%Y-%m-%d %H:%M").unwrap();

        result.push(LogLine::new(timestamp, splits[1].trim().to_string()));
    }

    result
}
