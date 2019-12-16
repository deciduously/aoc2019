use std::fmt;
pub type Int = i64;

const VALS_PER_OPCODE: usize = 4;
const MAX_INPUT: Int = 99;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Opcode {
    Add(Int, Int, Int),
    Multiply(Int, Int, Int),
    Terminate,
    Unknown,
}

impl Opcode {
    // expects 1 or more Ints, returns an Opcode
    fn new(ints: &[Int]) -> Self {
        if ints.is_empty() {
            panic!("Opcode::new() passed an empty slice!")
        } else {
            use Opcode::*;
            match ints[0] {
                1 => Add(ints[1], ints[2], ints[3]),
                2 => Multiply(ints[1], ints[2], ints[3]),
                99 => Terminate,
                _ => Unknown,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl Default for ParameterMode {
    fn default() -> Self {
        ParameterMode::Position
    }
}

#[derive(Debug, Default)]
pub struct IntcodeComputer {
    current_idx: usize,
    current_mode: ParameterMode,
    program: String,
    tape: Vec<Int>,
}

impl IntcodeComputer {
    pub fn new(input: &str) -> Self {
        let mut ret = Self::default();
        ret.program = input.to_string();
        ret.init_tape();
        ret
    }
    pub fn fix_1202bug(&mut self) {
        self.enter_inputs(12, 2);
    }
    pub fn execute(&mut self) {
        let mut running = true;
        while running {
            let opcode = self.get_opcode();
            use Opcode::*;
            match opcode {
                Add(lhs, rhs, dest) => {
                    self.set_value_at(
                        dest as usize,
                        self.get_value_at(lhs as usize) + self.get_value_at(rhs as usize),
                    );
                    self.current_idx += VALS_PER_OPCODE;
                }
                Multiply(lhs, rhs, dest) => {
                    self.set_value_at(
                        dest as usize,
                        self.get_value_at(lhs as usize) * self.get_value_at(rhs as usize),
                    );
                    self.current_idx += VALS_PER_OPCODE;
                }
                Terminate => running = false,
                Unknown => panic!("Expected opcode!!!"),
            }
        }
    }
    pub fn locate_target(&mut self, target: Int) -> (Int, Int) {
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
    pub fn result(&self) -> Int {
        self.get_value_at(0)
    }
    fn enter_inputs(&mut self, noun: Int, verb: Int) {
        self.tape[1] = noun;
        self.tape[2] = verb;
    }
    fn get_opcode(&self) -> Opcode {
        let mut ret = Vec::new();
        if self.get_value_at(self.current_idx) == 99 {
            Opcode::Terminate
        } else {
            for i in 0..VALS_PER_OPCODE {
                ret.push(self.get_value_at(self.current_idx + i));
            }
            Opcode::new(&ret)
        }
    }
    fn get_value_at(&self, pos: usize) -> Int {
        self.tape[pos]
    }
    fn init_tape(&mut self) {
        self.tape = self
            .program
            .split(',')
            .map(|s| s.parse::<Int>())
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap())
            .collect();
    }
    fn reset(&mut self) {
        self.init_tape();
        self.current_idx = 0;
    }
    fn set_value_at(&mut self, pos: usize, new_value: Int) {
        self.tape[pos] = new_value;
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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_v0_day2() {
        assert_eq!(intcode("1,0,0,0,99", false).1, "2,0,0,0,99");
        assert_eq!(intcode("2,3,0,3,99", false).1, "2,3,0,6,99");
        assert_eq!(intcode("2,4,4,5,99,0", false).1, "2,4,4,5,99,9801");
        assert_eq!(
            intcode("1,1,1,4,99,5,6,0,99", false).1,
            "30,1,1,4,2,5,6,0,99"
        );
        assert_eq!(
            intcode("1,9,10,3,2,3,11,0,99,30,40,50", false).1,
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
    }
    #[test]
    fn test_v1_day5() {
        // assert_eq!(intcode("1002,4,3,4,33"))
    }
}
