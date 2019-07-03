mod day1;
mod day2;
mod utils;

fn main() {
    println!("Advent of code 2019");

    compute_day1();
    compute_day2();
}

fn compute_day1() {
    let path = "K:/git/advent-of-code/src/day1/data.txt";

    let first_result = day1::compute_first(path);
    println!("Day 1, first result {0}", first_result);

    let second_result = day1::compute_second(path);
    println!("Day 1, second result {0}", second_result);
}

fn compute_day2(){
    let path = "K:/git/advent-of-code/src/day2/data.txt";

    let first_result = day2::compute_first(path);
    println!("Day 2, first result {0}", first_result);
}