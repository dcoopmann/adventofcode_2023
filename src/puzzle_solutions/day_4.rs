use crate::Problem;

pub struct DayFour;

impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {
        let mut result = 0;
        let lines = input.lines().collect::<Vec<_>>();
        for line in lines {
            let games = line.split(':').collect::<Vec<_>>()[1];
            let numbers = games.trim().split('|').collect::<Vec<_>>();
            let win = numbers[0].trim().split(' ').collect::<Vec<_>>();
            let lot = numbers[1].trim().split(' ').collect::<Vec<_>>();

            let l = win.len();
            let mut game_match = 0;

            for ticket in lot {
                if ticket.is_empty() {
                } else {
                    for i in 0..l {
                        if win[i].is_empty() {
                        } else {
                            if win[i] == ticket {
                                game_match += 1;
                            }
                        }
                    }
                }
            }

            let mut t = 0;
            if game_match > 0 {
                for i in 1..game_match + 1 {
                    if i == 1 || i == 2 {
                        t += 1
                    } else {
                        t = t * 2
                    }
                }
            }
            result += t;
        }

        result.to_string()
    }
    fn part_two(&self, input: &str) -> String {
        let mut result = 0;
        let lines = input.lines().collect::<Vec<_>>();

        let mut matches = Vec::new();

        for line in lines {
            let games = line.split(':').collect::<Vec<_>>()[1];
            let numbers = games.trim().split('|').collect::<Vec<_>>();
            let win = numbers[0].trim().split(' ').collect::<Vec<_>>();
            let lot = numbers[1].trim().split(' ').collect::<Vec<_>>();

            let l = win.len();
            let mut game_match = 0;

            for ticket in lot {
                if ticket.is_empty() {
                } else {
                    for i in 0..l {
                        if win[i].is_empty() {
                        } else {
                            if win[i] == ticket {
                                game_match += 1;
                            }
                        }
                    }
                }
            }

            matches.push(game_match)
        }

        let mut orig_count = Vec::new();

        for _i in 0..matches.len() {
            orig_count.push(1)
        }

        for i in 0..matches.len() {
            for _c in 1..orig_count[i] + 1 {
                if matches[i] > 0 {
                    for n in 1..matches[i] + 1 {
                        if i + n < matches.len() {
                            orig_count[i + n] += 1;
                        }
                    }
                }
            }
        }
        for num in orig_count {
            result += num
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let day = DayFour {};
        assert_eq!(day.part_one(input), "13");
    }
    #[test]
    fn test_part_two() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let day = DayFour {};
        assert_eq!(day.part_two(input), "30");
    }
}
