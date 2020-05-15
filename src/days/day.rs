use super::day_result::DayResult;

/// Задания в рамках одного дня
pub trait Day {

    /// Получить название
    fn get_name(&self) -> String;

    /// Вычислить результат первого задания для input
    fn compute_first(&self, input: &Vec<String>) -> DayResult;

    /// Вычислить результат второго задания для input
    fn compute_second(&self, input: &Vec<String>) -> DayResult;
}
