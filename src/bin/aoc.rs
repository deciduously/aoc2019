use aoc2019::*;
use std::env::args;

fn main() {
    if let Some(day) = args().nth(1) {
        match day.as_str() {
            "1" => day1::run(),
            "2" => day2::run(),
            _ => eprintln!("Currently implemented: 1, 2"),
        }
    } else {
        eprintln!("You must select a day to run");
    }
}
