use crate::Problem;

pub struct DayTwo;

struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

// impl Cubes {
//     fn new(red: u32, green: u32, blue: u32) -> Self {
//         return Cubes { red, green, blue };
//     }

//     fn is_in(&self, other: Cubes) -> bool {
//         return self.red <= other.red && self.green <= other.green && self.blue <= other.blue;
//     }
// }

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
            println!("----------------Loop-----------------");
            let parts = line.trim().split(':').collect::<Vec<_>>();
            let id = parts[0].split(' ').collect::<Vec<_>>()[1]
                .parse::<u64>()
                .unwrap();
            let rounds = parts[1].trim().split(';').collect::<Vec<_>>();

            println!("id: {}", id);
            println!("rounds: {:?}", rounds);
            let mut possible = true;

            for round in rounds {
                println!("round: {}", round);

                let vals = round.trim().split(',').collect::<Vec<_>>();

                println!("Vals: {:?}", vals);

                for val in vals {
                    let t = val.trim().split(' ').collect::<Vec<_>>();
                    println!("t: {:?}", t);

                    if t[1].starts_with("red") {
                        println!("matched red, val0: {}", t[0]);
                        if t[0].parse::<u32>().unwrap() > game_config.red {
                            possible = false;
                            break;
                        }
                    }
                    if t[1].starts_with("green") {
                        println!("matched green, val0: {}", t[0]);
                        if t[0].parse::<u32>().unwrap() > game_config.green {
                            possible = false;
                            break;
                        }
                    }
                    if t[1].starts_with("blue") {
                        println!("matched blue, val0: {}", t[0]);

                        if t[0].parse::<u32>().unwrap() > game_config.blue {
                            possible = false;
                            break;
                        }
                    }
                }
            }

            println!("Possible: {}", possible);
            if possible {
                result += id
            }
        }

        result.to_string()
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
        assert_eq!(day.part_two(input), "2541");
    }
}
