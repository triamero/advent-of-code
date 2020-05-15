extern crate advent_of_code;

mod compute_first {

    use advent_of_code::days::{day::Day, day1::Day1};
    use advent_of_code::utils;

    #[test]
    fn compute_first_sample1() {
        let data = utils::read_lines(&"tests/day1/data/first_sample1.txt");

        let result = Day1().compute_first(&data);

        assert_eq!(3, result.ival.unwrap());
    }

    #[test]
    fn compute_first_sample2() {
        let data = utils::read_lines(&"tests/day1/data/first_sample2.txt");

        let result = Day1().compute_first(&data);

        assert_eq!(0, result.ival.unwrap());
    }

    #[test]
    fn compute_first_sample3() {

        let data = utils::read_lines(&"tests/day1/data/first_sample3.txt");

        let result = Day1().compute_first(&data);

        assert_eq!(-6, result.ival.unwrap());
    }
}
