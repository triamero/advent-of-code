extern crate advent_of_code;

mod compute_first {

    use advent_of_code::days::{day::Day, day4::Day4};
    use advent_of_code::utils;

    #[test]
    fn compute_first_sample1() {
        let data = utils::read_lines(&"tests/day4/data/first_sample1.txt");

        let result = Day4().compute_first(&data);

        assert_eq!(240, result.ival.unwrap());
    }
}