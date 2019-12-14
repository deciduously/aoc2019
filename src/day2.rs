use super::get_puzzle_string;
use std::fmt;

type Int = i32;

#[derive(Debug, Clone, Copy, PartialEq)]
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
            .split(',')
            .map(|s| s.parse::<Int>())
            .filter(|res| res.is_ok()) // TODO is there a cleaner way than jut dropping mismatches?
            .map(|res| res.unwrap())
            .collect();
        Self {
            current_idx: 0,
            program,
        }
    }
    fn get_value_at(&self, pos: usize) -> Int {
        self.program[pos]
    }
    fn set_value_at(&mut self, pos: usize, new_value: Int) {
        self.program[pos] = new_value;
    }
    fn fix_1202bug(&mut self) {
        self.program[1] = 12;
        self.program[2] = 2;
    }
    // Returns true if still executing
    fn execute(&mut self) -> bool {
        // get current code
        if let Some(opcode) = Opcode::from_int(self.get_value_at(self.current_idx)) {
            use Opcode::*;
            match opcode {
                Add | Multiply => {
                    let lhs = self.get_value_at(self.current_idx + 1);
                    let rhs = self.get_value_at(self.current_idx + 2);
                    let dest = self.get_value_at(self.current_idx + 3);
                    if opcode == Opcode::Add {
                        self.set_value_at(
                            dest as usize,
                            self.get_value_at(lhs as usize) + self.get_value_at(rhs as usize),
                        );
                    } else {
                        self.set_value_at(
                            dest as usize,
                            self.get_value_at(lhs as usize) * self.get_value_at(rhs as usize),
                        );
                    }
                    // Advance to next opcode
                    self.current_idx += 4;
                    true
                }
                Terminate => false,
            }
        } else {
            panic!("expected opcode!!!")
        }
    }
}

impl fmt::Display for IntcodeComputer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        for cell in &self.program {
            ret.push_str(&format!("{},", cell));
        }
        // trim last ,
        write!(f, "{}", &ret[0..ret.len() - 1])
    }
}

fn intcode(input: &str, buggy: bool) -> String {
    let mut computer = IntcodeComputer::new(input);
    if buggy {
        computer.fix_1202bug();
    }
    let mut running = true;
    while running {
        running = computer.execute();
    }
    computer.to_string()
}

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
