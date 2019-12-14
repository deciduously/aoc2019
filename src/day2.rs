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
    fn test_part1() {
        assert_eq!(intcode("1,0,0,0,99", false).1, "2,0,0,0,99");
        assert_eq!(intcode("2,3,0,3,99", false).1, "2,3,0,6,99");
        assert_eq!(intcode("2,4,4,5,99,0", false).1, "2,4,4,5,99,9801");
        assert_eq!(intcode("1,1,1,4,99,5,6,0,99", false).1, "30,1,1,4,2,5,6,0,99");
        assert_eq!(
            intcode("1,9,10,3,2,3,11,0,99,30,40,50", false).1,
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
    }
}
