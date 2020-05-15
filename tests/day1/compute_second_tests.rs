extern crate advent_of_code;

mod compute_second {

    use advent_of_code::days::{day::Day, day1::Day1};
    use advent_of_code::utils;

    #[test]
    fn compute_second_sample1() {
        let data = utils::read_lines(&"tests/day1/data/second_sample1.txt");

        let result = Day1().compute_second(&data);

        assert_eq!(0, result.ival.unwrap());
    }

    #[test]
    fn compute_second_sample2() {

        let data = utils::read_lines(&"tests/day1/data/second_sample2.txt");

        let result = Day1().compute_second(&data);

        assert_eq!(10, result.ival.unwrap());
    }

    #[test]
    fn compute_second_sample3() {

        let data = utils::read_lines(&"tests/day1/data/second_sample3.txt");

        let result = Day1().compute_second(&data);

        assert_eq!(5, result.ival.unwrap());
    }

    #[test]
    fn compute_second_sample4() {

        let data = utils::read_lines(&"tests/day1/data/second_sample4.txt");

        let result = Day1().compute_second(&data);

        assert_eq!(14, result.ival.unwrap());
    }
}
