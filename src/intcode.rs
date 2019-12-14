use std::fmt;
type Int = u32;

const VALS_PER_OPCODE: usize = 4;
const MAX_INPUT: Int = 99;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Opcode {
    Add,
    Multiply,
    Terminate,
    Unknown,
}

impl From<Int> for Opcode {
    fn from(i: Int) -> Self {
        use Opcode::*;
        match i {
            1 => Add,
            2 => Multiply,
            99 => Terminate,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Default)]
struct IntcodeComputer {
    current_idx: usize,
    program: String,
    tape: Vec<Int>,
}

impl IntcodeComputer {
    fn new(input: &str) -> Self {
        let mut ret = Self::default();
        ret.program = input.to_string();
        ret.init_tape();
        ret
    }
    fn init_tape(&mut self) {
        self.tape = self
            .program
            .split(',')
            .map(|s| s.parse::<u32>())
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .collect();
    }
    fn reset(&mut self) {
        self.init_tape();
        self.current_idx = 0;
    }
    fn get_value_at(&self, pos: usize) -> Int {
        self.tape[pos]
    }
    fn set_value_at(&mut self, pos: usize, new_value: Int) {
        self.tape[pos] = new_value;
    }
    fn fix_1202bug(&mut self) {
        self.enter_inputs(12, 2);
    }
    // Returns true if still executing
    fn execute(&mut self) {
        let mut running = true;
        while running {
            let opcode = Opcode::from(self.get_value_at(self.current_idx));
            use Opcode::*;
            match opcode {
                Add | Multiply => {
                    // TODO binop macro!
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
                    self.current_idx += VALS_PER_OPCODE;
                }
                Terminate => running = false,
                Unknown => panic!("Expected opcode!!!"),
            }
        }
    }
    fn enter_inputs(&mut self, noun: Int, verb: Int) {
        self.tape[1] = noun;
        self.tape[2] = verb;
    }
    fn locate_target(&mut self, target: Int) -> (Int, Int) {
        for noun in 0..=MAX_INPUT {
            for verb in 0..=MAX_INPUT {
                self.reset();
                self.enter_inputs(noun, verb);
                self.execute();
                if self.result() == target {
                    return (noun, verb);
                }
            }
        }
        panic!("Tried all possible int pairs - no match");
    }
    fn result(&self) -> Int {
        self.get_value_at(0)
    }
}

impl fmt::Display for IntcodeComputer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        for cell in &self.tape {
            ret.push_str(&format!("{},", cell));
        }
        // trim last ','
        write!(f, "{}", &ret[0..ret.len() - 1])
    }
}

pub fn intcode(input: &str, buggy: bool) -> (Int, String) {
    let mut computer = IntcodeComputer::new(input);
    if buggy {
        computer.fix_1202bug();
    }
    computer.execute();
    (computer.result(), computer.to_string())
}

pub fn find_inputs(input: &str, target: Int) -> Int {
    let mut computer = IntcodeComputer::new(input);
    let (noun, verb) = computer.locate_target(target);
    100 * noun + verb
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_intcode() {
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