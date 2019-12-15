mod intcode;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub use intcode::{Int, IntcodeComputer};

use std::{
    fs::File,
    io::{self, BufReader, ErrorKind::*, Read},
};

const INPUT_DIR: &str = "inputs";

fn get_puzzle_string(day: u8) -> Result<String, io::Error> {
    let filename = format!("{}/day{}.txt", INPUT_DIR, day);
    let mut ret = String::new();

    if let Ok(file) = File::open(&filename) {
        // Read it from disk
        let mut buf = BufReader::new(file);
        buf.read_to_string(&mut ret)?;
        Ok(ret)
    } else {
        Err(io::Error::new(InvalidData, format!("You need to log in to adventofcode.com via a web browser and download the Day {} puzzle input!", day)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_get_puzzle_string() {
        assert_eq!(&get_puzzle_string(0).unwrap(), "Test\nFile\n")
    }
}
