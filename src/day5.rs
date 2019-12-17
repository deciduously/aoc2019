use crate::*;

pub fn run() {
    intcode::intcode(&get_puzzle_string(5).unwrap(), false);
}