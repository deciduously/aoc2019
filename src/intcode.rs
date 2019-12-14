use std::fmt;
pub type Int = u32;

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
pub struct IntcodeComputer {
    current_idx: usize,
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
    fn get_value_at(&self, pos: usize) -> Int {
        self.tape[pos]
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

#[cfg(test)]
mod test {
    //use super::*;
    //use pretty_assertions::assert_eq;
}