pub trait Day {

    fn get_name(&self) -> String;

    fn compute_first(&self, input: &Vec<String>) -> i32;

    fn compute_second(&self, input: &Vec<String>) -> i32;
}
