use adventofcode_2023::puzzle_solutions::day_1::DayOne;
use adventofcode_2023::puzzle_solutions::day_2::DayTwo;
use adventofcode_2023::puzzle_solutions::day_4::DayFour;
use adventofcode_2023::puzzle_solutions::day_6::DaySix;
use adventofcode_2023::puzzle_solutions::day_template::DayTemplate;
use adventofcode_2023::Problem;

use clap::Parser;

use std::fs;

#[derive(Parser, Debug)]
#[command(author, version,  about, long_about=None)]
struct Args {
    ///Day
    #[arg(short, long)]
    day: usize,
    ///Part
    #[arg(short, long)]
    part: usize,
}

fn select_day(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne {})),
        2 => Some(Box::new(DayTwo {})),
        4 => Some(Box::new(DayFour {})),
        6 => Some(Box::new(DaySix {})),
        _ => Some(Box::new(DayTemplate {})),
    }
}

fn load_puzzle_input(day: usize) -> String {
    let path = format!("puzzle_input/day_{}.txt", day);
    return fs::read_to_string(path).unwrap_or_default();
}

fn main() {
    let args = Args::parse();
    println!("Solving Day: {} Part {}", args.day, args.part);

    let day = select_day(args.day).unwrap();

    if args.part == 1 {
        println!(
            "Result Part One: {}",
            day.part_one(&load_puzzle_input(args.day))
        );
    } else if args.part == 2 {
        println!(
            "Result Part Two: {}",
            day.part_two(&load_puzzle_input(args.day))
        );
    } else {
        println!("There are only part one or two, not part {}", args.part)
    }
}
