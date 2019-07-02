pub mod first;
pub mod second;

pub fn compute_first(path_to_data: &str) -> i32 {
    return first::compute(path_to_data);
}

pub fn compute_second(path_to_data: &str) -> i32 {
    return second::compute(path_to_data);
}