mod days;
mod utils;
use days::day::Day;
use days::day1::Day1;
use days::day2::Day2;

fn main() {
    println!("Advent of code 2019");

    let days: Vec<&dyn Day> = vec![
        &Day1(),
        &Day2()
    ];

    println!("The current directory is {}", std::env::current_dir().unwrap().display());

    for day in &days {
        let name = day.get_name();

        let mut path = String::from("src/resources/");

        path.insert_str(path.len(), &format!("{}.txt", name));

        println!("Path is {}", path);

        let input = utils::read_lines(&path);

        let value1 = day.compute_first(&input);
        let value2 = day.compute_second(&input);

        println!("{} first result: {}", name, value1);
        println!("{} second result: {}", name, value2);
    }
}
