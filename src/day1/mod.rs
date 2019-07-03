pub mod first;
pub mod second;

pub fn compute_first(path_to_data: &str) -> i32 {

    let input: Vec<String> = crate::utils::utils::read_lines(path_to_data);

    return first::compute(input);
}

pub fn compute_second(path_to_data: &str) -> i32 {

    let input: Vec<String> = crate::utils::utils::read_lines(path_to_data);

    return second::compute(input);
}