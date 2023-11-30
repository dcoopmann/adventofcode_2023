use crate::Problem;

pub struct DayTemplate;

impl Problem for DayTemplate {
    fn part_one(&self, _input: &str) -> String {
        "This is just a template part one".to_string()
    }
    fn part_two(&self, _input: &str) -> String {
        "This is just a template part two".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "This is just some input part one";

        let day = DayTemplate {};
        assert_eq!(day.part_one(input), "This is just a template part one");
    }
    #[test]
    fn test_part_two() {
        let input = "This is just some input part two";

        let day = DayTemplate {};
        assert_eq!(day.part_two(input), "This is just a template part two");
    }
}
