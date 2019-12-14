use std::fmt;
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Int(i32);

impl From<i32> for Int {
    fn from(i: i32) -> Self {
        Self(i)
    }
}

impl From<Int> for i32 {
    fn from(i: Int) -> Self {
        i.0
    }
}

impl Add for Int {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<i32> for Int {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs;
    }
}

impl Mul for Int {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl From<Int> for usize {
    fn from(i: Int) -> Self {
        i.0 as usize
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

const VALS_PER_OPCODE: usize = 4;

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
        match i32::from(i) {
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
        let mut ret = Self {
            current_idx: 0,
            program: input.into(),
            tape: vec![],
        };
        ret.reset_tape();
        ret
    }
    fn reset_tape(&mut self) {
        self.tape = self
            .program
            .split(',')
            .map(|s| s.parse::<i32>())
            .filter(|res| res.is_ok())
            .map(|res| Int::from(res.unwrap()))
            .collect();
    }
    fn get_value_at(&self, pos: usize) -> Int {
        self.tape[pos]
    }
    fn set_value_at(&mut self, pos: usize, new_value: Int) {
        self.tape[pos] = new_value;
    }
    fn fix_1202bug(&mut self) {
        self.enter_inputs(12.into(), 2.into());
    }
    // Returns true if still executing
    fn execute(&mut self) -> bool {
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
                        dest.into(),
                        self.get_value_at(lhs.into()) + self.get_value_at(rhs.into()),
                    );
                } else {
                    self.set_value_at(
                        dest.into(),
                        self.get_value_at(lhs.into()) * self.get_value_at(rhs.into()),
                    );
                }
                // Advance to next opcode
                self.current_idx += VALS_PER_OPCODE;
                true
            }
            Terminate => false,
            Unknown => panic!("Expected opcode!!!"),
        }
    }
    fn enter_inputs(&mut self, noun: Int, verb: Int) {
        self.tape[1] = noun;
        self.tape[2] = verb;
    }
    fn locate_target(&mut self, target: Int) -> (Int, Int) {
        let mut result;
        let mut noun = Int(0);
        let mut verb = Int(0);
        let mut flip_noun = true;
        loop {
            self.reset_tape();
            self.enter_inputs(noun, verb);
            self.execute();
            result = self.tape[0];
            if result == target {
                break;
            }
            // change parameters
            if flip_noun {
                noun += 1;
            } else {
                verb += 1;
            }
            flip_noun = !flip_noun;
        }
        (noun, verb)
    }
}

impl fmt::Display for IntcodeComputer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        for cell in &self.tape {
            ret.push_str(&format!("{},", cell));
        }
        // trim last ,
        write!(f, "{}", &ret[0..ret.len() - 1])
    }
}

pub fn intcode(input: &str, buggy: bool) -> String {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::get_puzzle_string;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_locate_target() {
        let mut computer = IntcodeComputer::new(&get_puzzle_string(2));
        //assert_eq!(computer.locate_target(Int(1202)), (Int(12), Int(2)))
    }
}
