use crate::Problem;

pub struct DayEight;

impl Problem for DayEight {
    fn part_one(&self, input: &str) -> String {
        let mut input = input.lines();
        let instructions = input.next().unwrap().trim().chars().collect::<Vec<_>>();
        input.next(); // pass empty value

        let map = input.map(|l| Node::new(l.trim())).collect::<Vec<_>>();

        println!("Instructions: {:?}", instructions);

        let mut result = 0;

        let mut next = &"AAA".to_string();

        'outer: loop {
            for i in &instructions {
                for node in &map {
                    if node.name == *next {
                        if &node.name == "ZZZ" {
                            break 'outer;
                        }
                        if i == &'L' {
                            next = &node.left
                        } else {
                            next = &node.right
                        }
                        break;
                    }
                }
                result += 1;
            }
            // break;
        }

        result.to_string()
    }
    fn part_two(&self, _input: &str) -> String {
        "This is just a template part two".to_string()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(line: &str) -> Self {
        Node {
            name: line[0..3].to_string(),
            left: line[7..10].to_string(),
            right: line[12..15].to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

        let day = DayEight {};
        assert_eq!(day.part_one(input), "2");
    }

    #[test]
    fn test_part_one_again() {
        let input = "LLR

        BBB = (AAA, ZZZ)
        AAA = (BBB, BBB)
        ZZZ = (ZZZ, ZZZ)";

        let day = DayEight {};
        assert_eq!(day.part_one(input), "6");
    }
    #[test]
    fn test_part_two() {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        let day = DayEight {};
        assert_eq!(day.part_two(input), "This is just a template part two");
    }
}
