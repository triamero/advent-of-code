extern crate advent_of_code;

#[test]
fn first_sample1() {

    let result = advent_of_code::day1::compute_first(&"K:/git/advent-of-code/tests/day1/data/first_sample1.txt");

    assert_eq!(3, result);
}

#[test]
fn first_sample2() {

    let result = advent_of_code::day1::compute_first(&"K:/git/advent-of-code/tests/day1/data/first_sample2.txt");

    assert_eq!(0, result);
}


#[test]
fn first_sample3() {

    let result = advent_of_code::day1::compute_first(&"K:/git/advent-of-code/tests/day1/data/first_sample3.txt");

    assert_eq!(-6, result);
}

#[test]
fn second_sample1() {

    let result = advent_of_code::day1::compute_second(&"K:/git/advent-of-code/tests/day1/data/second_sample1.txt");

    assert_eq!(0, result);
}

#[test]
fn second_sample2() {

    let result = advent_of_code::day1::compute_second(&"K:/git/advent-of-code/tests/day1/data/second_sample2.txt");

    assert_eq!(10, result);
}


#[test]
fn second_sample3() {

    let result = advent_of_code::day1::compute_second(&"K:/git/advent-of-code/tests/day1/data/second_sample3.txt");

    assert_eq!(5, result);
}

#[test]
fn second_sample4() {

    let result = advent_of_code::day1::compute_second(&"K:/git/advent-of-code/tests/day1/data/second_sample4.txt");

    assert_eq!(14, result);
}