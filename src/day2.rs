use super::{get_puzzle_string, intcode};

pub fn run() {
    println!("Day 2");
    println!("{}", intcode(&get_puzzle_string(2), true));
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_sample() {
        assert_eq!(intcode("1,0,0,0,99", false), "2,0,0,0,99");
        assert_eq!(intcode("2,3,0,3,99", false), "2,3,0,6,99");
        assert_eq!(intcode("2,4,4,5,99,0", false), "2,4,4,5,99,9801");
        assert_eq!(intcode("1,1,1,4,99,5,6,0,99", false), "30,1,1,4,2,5,6,0,99");
        assert_eq!(
            intcode("1,9,10,3,2,3,11,0,99,30,40,50", false),
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
    }
}
