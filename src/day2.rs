use super::*;

pub fn run() {
    println!("{}", intcode(&get_puzzle_string(2), true).0);
    println!("{}", find_inputs(&get_puzzle_string(2), 19690720));
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_solutions() {
        assert_eq!(intcode(&get_puzzle_string(2), true).0, 4945026);
        assert_eq!(find_inputs(&get_puzzle_string(2), 19690720), 5296);
    }
}
