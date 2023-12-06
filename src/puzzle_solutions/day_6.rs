use crate::Problem;

pub struct DaySix;

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        let mut result = 1;

        let mut lines = input.lines();
        let times = lines.next().unwrap().trim().split(':').collect::<Vec<_>>()[1]
            .trim()
            .split(' ')
            .filter(|x| x.len() > 0)
            .collect::<Vec<_>>();

        let records = lines.next().unwrap().trim().split(':').collect::<Vec<_>>()[1]
            .trim()
            .split(' ')
            .filter(|x| x.len() > 0)
            .collect::<Vec<_>>();

        assert_eq!(times.len(), records.len());
        for i in 0..times.len() {
            result *= calculate_race(times[i].parse().unwrap(), records[i].parse().unwrap())
        }

        result.to_string()
    }
    fn part_two(&self, input: &str) -> String {
        let mut lines = input.lines();

        let time = lines.next().unwrap().trim().split(':').collect::<Vec<_>>()[1]
            .trim()
            .replace(' ', "")
            .parse()
            .unwrap();
        let record = lines.next().unwrap().trim().split(':').collect::<Vec<_>>()[1]
            .trim()
            .replace(' ', "")
            .parse()
            .unwrap();

        println!("time: {} record: {}", time, record);

        let result = calculate_race(time, record);

        result.to_string()
    }
}

fn calculate_race(time: u64, dist_record: u64) -> u64 {
    let mut possible_record_breakers = 0;
    for t in 0..time {
        if t * (time - t) > dist_record {
            possible_record_breakers += 1
        }
    }
    return possible_record_breakers;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_race() {
        assert_eq!(4, calculate_race(7, 9));
        assert_eq!(8, calculate_race(15, 40));
        assert_eq!(9, calculate_race(30, 200));
    }

    #[test]
    fn test_part_one() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        let day = DaySix {};
        assert_eq!(day.part_one(input), "288");
    }
    #[test]
    fn test_part_two() {
        let input = "Time:      71530
        Distance:  940200";

        let day = DaySix {};
        assert_eq!(day.part_two(input), "71503");
    }
}
