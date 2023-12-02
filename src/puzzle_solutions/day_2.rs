use crate::Problem;

pub struct DayTwo;

#[derive(Default, Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn power(&self) -> u32 {
        return self.red * self.green * self.blue;
    }
}

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let game_config = Cubes {
            red: 12,
            green: 13,
            blue: 14,
        };

        let lines = input.lines().collect::<Vec<_>>();
        let mut result = 0;

        for line in lines {
            let parts = line.trim().split(':').collect::<Vec<_>>();
            let id = parts[0].split(' ').collect::<Vec<_>>()[1]
                .parse::<u64>()
                .unwrap();
            let rounds = parts[1].trim().split(';').collect::<Vec<_>>();

            let mut possible = true;

            for round in rounds {
                let vals = round.trim().split(',').collect::<Vec<_>>();

                for val in vals {
                    let t = val.trim().split(' ').collect::<Vec<_>>();

                    if t[1].starts_with("red") {
                        if t[0].parse::<u32>().unwrap() > game_config.red {
                            possible = false;
                            break;
                        }
                    }
                    if t[1].starts_with("green") {
                        if t[0].parse::<u32>().unwrap() > game_config.green {
                            possible = false;
                            break;
                        }
                    }
                    if t[1].starts_with("blue") {
                        if t[0].parse::<u32>().unwrap() > game_config.blue {
                            possible = false;
                            break;
                        }
                    }
                }
            }

            if possible {
                result += id
            }
        }

        result.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let lines = input.lines().collect::<Vec<_>>();
        let mut result = 0;

        for line in lines {
            let mut cube_counter = Cubes::default();

            let parts = line.trim().split(':').collect::<Vec<_>>();
            let rounds = parts[1].trim().split(';').collect::<Vec<_>>();

            for round in rounds {
                let vals = round.trim().split(',').collect::<Vec<_>>();

                for val in vals {
                    let t = val.trim().split(' ').collect::<Vec<_>>();

                    if t[1].starts_with("red") {
                        if t[0].parse::<u32>().unwrap() > cube_counter.red {
                            cube_counter.red = t[0].parse::<u32>().unwrap();
                        }
                    }
                    if t[1].starts_with("green") {
                        if t[0].parse::<u32>().unwrap() > cube_counter.green {
                            cube_counter.green = t[0].parse::<u32>().unwrap();
                        }
                    }
                    if t[1].starts_with("blue") {
                        if t[0].parse::<u32>().unwrap() > cube_counter.blue {
                            cube_counter.blue = t[0].parse::<u32>().unwrap();
                        }
                    }
                }
            }

            result += cube_counter.power()
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let day = DayTwo {};
        assert_eq!(day.part_one(input), "8");
    }
    #[test]
    fn test_part_two() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let day = DayTwo {};
        assert_eq!(day.part_two(input), "2286");
    }
}
