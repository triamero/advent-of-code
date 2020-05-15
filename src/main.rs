mod days;
mod utils;
use days::day_result::DayResult;
use days::day::Day;
use days::day1::Day1;
use days::day2::Day2;
use days::day3::Day3;

fn main() {
    println!("Advent of code 2019");

    let days: Vec<&dyn Day> = vec![&Day1(), &Day2(), &Day3()];

    println!(
        "The current directory is {}",
        std::env::current_dir().unwrap().display()
    );

    for day in &days {
        let name = day.get_name();

        let mut path = String::from("src/resources/");

        path.insert_str(path.len(), &format!("{}.txt", name));

        let input = utils::read_lines(&path);

        let value1: DayResult = day.compute_first(&input);
        let value2: DayResult = day.compute_second(&input);

        print_result(format!("{} first", name), value1);
        print_result(format!("{} second", name), value2);
    }
}

fn print_result(prefix: String, result: DayResult) {

    if let Some(val) = result.ival {
        print_i32(&prefix, val);
    }

    if let Some(val) = result.sval {
        print_string(&prefix, &val);
    }
}

fn print_i32(prefix: &str, value: i32) {
    println!("{} result is {}", prefix, value);
}

fn print_string(prefix: &str, value: &str) {
    println!("{} result is {}", prefix, value);
}
