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
                if char.is_numeric(){
                    if first {
                            a = char;
                            first = false
                        }
                        else {
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
    fn part_two(&self, _input: &str) -> String {
        "This is just a template part two".to_string()
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
        7pqrstsixteen";

        let day = DayOne {};
        assert_eq!(day.part_two(input), "281");
    }
}
