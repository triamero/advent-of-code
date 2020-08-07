extern crate advent_of_code;

mod compute_first {

    use advent_of_code::days::{day::Day, day5::Day5};
    use advent_of_code::utils;

    #[test]
    fn compute_first_sample1() {
        let data = utils::read_lines(&"tests/day5/data/first_sample1.txt");

        let result = Day5().compute_first(&data);

        assert_eq!(10, result.ival.unwrap());
    }
}