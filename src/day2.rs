use super::*;
use crate::intcode::intcode;

pub fn find_inputs(input: &str, target: Int) -> Int {
    let mut computer = IntcodeComputer::new(input, vec![]);
    let (noun, verb) = computer.locate_target(target).unwrap();
    100 * noun + verb
}

pub fn run() {
    println!(
        "{}",
        intcode(&get_puzzle_string(2).unwrap(), true, vec![]).0
    );
    println!(
        "{}",
        find_inputs(&get_puzzle_string(2).unwrap(), 19_690_720)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_solutions() {
        assert_eq!(
            intcode::intcode(&get_puzzle_string(2).unwrap(), true, vec![]).0,
            4945026
        );
        assert_eq!(
            find_inputs(&get_puzzle_string(2).unwrap(), 19_690_720),
            5296
        );
    }
}
