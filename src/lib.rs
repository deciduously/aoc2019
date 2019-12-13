pub mod day1;

use std::{
    fs::File,
    io::{BufReader, Read},
};

fn get_puzzle_string(day: u8) -> String {
    let filename = format!("inputs/day{}.txt", day);
    let file = File::open(filename).unwrap();
    let mut ret = String::new();
    let mut buf = BufReader::new(file);
    buf.read_to_string(&mut ret).unwrap();
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_get_puzzle_string() {
        assert_eq!(get_puzzle_string(0), "Test\nFile\n".to_string())
    }
}
