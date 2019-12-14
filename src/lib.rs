mod intcode;

pub mod day1;
pub mod day2;
pub mod day3;

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
        /*
        TODO howto auth???
        // Download it
        let url = format!("https://adventofcode.com/2019/day/{}/input", day);
        let ret = reqwest::get(&url)?.text()?;

        // Save it to disk for next time
        use std::io::Write;
        let bytes = ret.as_bytes();
        let mut cursor = 0;
        let mut new_file = File::create(&filename)?;
         while cursor < bytes.len() {
             let total_written = new_file.write(&bytes[cursor..])?;
             cursor += total_written;
         }
         */
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
