use crate::Problem;

pub struct DayOne;

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        let lines = input.lines().collect::<Vec<_>>();

        let mut result = 0;

        for line in lines {
            let mut first = true;
            let mut further = false;
            let mut a = char::default();
            let mut b = char::default();

            for char in line.trim().chars() {
                if char.is_numeric() {
                    if first {
                        a = char;
                        first = false
                    } else {
                        b = char;
                        further = true;
                    }
                }
            }
            let mut temp = String::new();
            if further {
                temp.push(a);
                temp.push(b);
                result += temp.parse::<u32>().unwrap();
            } else {
                temp.push(a);
                temp.push(a);
                result += temp.parse::<u32>().unwrap();
            }
        }

        result.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let lines = input.lines().collect::<Vec<_>>();

        let numbers = vec![
            Number::new("one", '1'),
            Number::new("two", '2'),
            Number::new("three", '3'),
            Number::new("four", '4'),
            Number::new("five", '5'),
            Number::new("six", '6'),
            Number::new("seven", '7'),
            Number::new("eight", '8'),
            Number::new("nine", '9'),
        ];

        let mut result = 0;
        for line in lines {
            let l = line.trim();
            let mut nums = Vec::new();
            for c in 0..l.len() {
                for number in &numbers {
                    if l[c..].starts_with(&number.literal) {
                        nums.push(&number.representation);
                    }
                    if l[c..].starts_with(number.representation) {
                        nums.push(&number.representation);
                    }
                }
            }

            let mut temp = String::new();

            temp.push(**nums.iter().next().unwrap());
            temp.push(**nums.iter().next_back().unwrap());

            result += temp.parse::<u32>().unwrap();
        }

        result.to_string()
    }
}

struct Number {
    literal: String,
    representation: char,
}

impl Number {
    fn new(literal: &str, representation: char) -> Self {
        Number {
            literal: literal.to_string(),
            representation: representation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let day = DayOne {};
        assert_eq!(day.part_one(input), "142");
    }
    #[test]
    fn test_part_two() {
        let input = "Ttwo1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        5foursix79625";

        let day = DayOne {};
        assert_eq!(day.part_two(input), "336");
    }

    #[test]
    fn test_part_two_t2() {
        let input = "1259one4gztcz
        jlmrvtxjphtwo2
        zrldjg2two
        sixdhsfvqrnseven5ms1
        gmrdkgqb8lmlpzjfflsbkjnntjc9eightcdpdq
        4nvxdvgjrnzdtblhxfsvdpvm3seven1tlkhfzmjqp
        dxjjbv4lvc39bscvppc91
        c6
        dvftxktvsonehcprkszlbfive1jxckpvknfthxnsm2
        drglpjtm1fivesixccr
        five35ksnljjnpdnine";

        let day = DayOne {};
        assert_eq!(day.part_two(input), "442");
    }
}
