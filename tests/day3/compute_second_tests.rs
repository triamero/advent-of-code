extern crate advent_of_code;

mod compute_second {

    use advent_of_code::days::{day::Day, day3::Day3};
    use advent_of_code::utils;

    #[test]
    fn compute_second_sample1() {
        let data = utils::read_lines(&"tests/day3/data/first_sample1.txt");

        let result = Day3().compute_second(&data);

        assert_eq!(3, result.ival.unwrap());
    }
}