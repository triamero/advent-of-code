mod compute_second_tests {
    
    use advent_of_code::days::{day::Day, day2::Day2};
    use advent_of_code::utils;

    #[test]
    fn compute_second_sample1() {

        let data = utils::read_lines(&"tests/day2/data/second_sample1.txt");

        let result = Day2().compute_second(&data);

        assert_eq!("fgij", result.sval.unwrap());
    }
}