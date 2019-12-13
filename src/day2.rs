use super::get_puzzle_string;

type Int = i32;

#[derive(Debug)]
enum Opcode {
    Add,
    Multiply,
    Terminate,
}

impl Opcode {
    fn from_int(i: Int) -> Option<Self> {
        use Opcode::*;
        match i {
            1 => Some(Add),
            2 => Some(Multiply),
            99 => Some(Terminate),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
struct IntcodeComputer {
    current_idx: usize,
    program: Vec<Int>,
}

impl IntcodeComputer {
    fn new(input: &str) -> Self {
        let program = input
            .split(",")
            .map(|s| s.parse::<Int>())
            .filter(|res| res.is_ok()) // TODO eww - is there a cleaner way than jut dropping mismatches?
            .map(|res| res.unwrap())
            .collect();
        Self {
            current_idx: 0,
            program,
        }
    }
}

fn intcode(input: &str) -> &str {
    let computer = IntcodeComputer::new(input);
    input
}

pub fn run() {
    println!("Day 2");
    println!("{}", intcode(&get_puzzle_string(2)));
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_sample() {
        assert_eq!(
            intcode("1,9,10,3,2,3,11,0,99,30,40,50"),
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
        assert_eq!(intcode("1,0,0,0,99"), "2,0,0,0,99");
        assert_eq!(intcode("2,3,0,3,99"), "2,3,0,6,99");
        assert_eq!(intcode("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        assert_eq!(intcode("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
    }
}
