use aoc2019::*;
use std::env::args;

const DAYS_IMPLEMENTED: u32 = 7;

fn main() {
    if let Some(day) = args().nth(1) {
        if let Ok(day) = day.parse::<u32>() {
            if day <= DAYS_IMPLEMENTED && day > 0 {
                println!("Day {}", day);
                match day {
                    1 => day1::run(),
                    2 => day2::run(),
                    3 => day3::run(),
                    4 => day4::run(),
                    5 => day5::run(),
                    6 => day6::run(),
                    7 => day7::run(),
                    _ => unreachable!(),
                }
            } else {
                eprintln!("Day must be between 1 and {} inclusive", DAYS_IMPLEMENTED);
            }
        } else {
            eprintln!("Day must be a number 1-{}", DAYS_IMPLEMENTED);
        }
    } else {
        eprintln!("You must select a day 1-{} to run", DAYS_IMPLEMENTED);
    }
}
