extern crate advent_of_code;

mod compute_first {

    use advent_of_code::days::{day::Day, day2::Day2};
    use advent_of_code::utils;

    #[test]
    fn first_sample1() {

        let data = utils::read_lines(&"tests/day2/data/first_sample1.txt");

        let result = Day2().compute_first(&data);

        assert_eq!(12, result.ival.unwrap());
    }
}
