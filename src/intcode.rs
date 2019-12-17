use std::{
    fmt,
    io::{self, ErrorKind::*},
};
pub type Int = isize;

const MAX_INPUT: Int = 99;

#[derive(Debug, Clone, Copy, PartialEq)]
enum OpcodeVariant {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    Terminate = 99,
}

impl OpcodeVariant {
    fn new(i: Int) -> Result<Self, io::Error> {
        use OpcodeVariant::*;
        match i % 100 {
            1 => Ok(Add),
            2 => Ok(Multiply),
            3 => Ok(Input),
            4 => Ok(Output),
            99 => Ok(Terminate),
            _ => Err(io::Error::new(
                InvalidInput,
                format!("Unknown opcode variant {}", i),
            )),
        }
    }
    fn instruction_len(self) -> usize {
        use OpcodeVariant::*;
        match self {
            Add | Multiply => 4,
            Input | Output => 2,
            Terminate => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Parameter {
    value: Int,
    mode: ParameterMode,
}

impl Parameter {
    fn new(value: Int, mode: Int) -> Result<Self, io::Error> {
        Ok(Self {
            value,
            mode: ParameterMode::new(mode)?,
        })
    }
}

#[derive(Debug)]
struct Opcode {
    variant: OpcodeVariant,
    parameters: Vec<Parameter>,
}

impl Opcode {
    fn new(variant: OpcodeVariant, parameters: Vec<Parameter>) -> Self {
        Self {
            variant,
            parameters,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

impl ParameterMode {
    fn new(i: Int) -> Result<Self, io::Error> {
        use ParameterMode::*;
        match i {
            0 => Ok(Position),
            1 => Ok(Immediate),
            _ => Err(io::Error::new(InvalidInput, "Unknown parameter mode")),
        }
    }
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
    pub fn execute(&mut self) -> Result<(), io::Error> {
        let mut running = true;
        while running {
            let opcode = self.get_opcode()?;
            use OpcodeVariant::*;
            match opcode.variant {
                Add => {
                    let lhs = self.read_parameter(opcode.parameters[0]);
                    let rhs = self.read_parameter(opcode.parameters[1]);
                    // dest is ALWAYS immediate??
                    let dest = opcode.parameters[2].value;
                    self.set_value_at(dest as usize, lhs + rhs);
                }
                Multiply => {
                    let lhs = self.read_parameter(opcode.parameters[0]);
                    let rhs = self.read_parameter(opcode.parameters[1]);
                    let dest = opcode.parameters[2].value;
                    self.set_value_at(dest as usize, lhs * rhs);
                }
                Input => {
                    print!("Enter value> ");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    self.set_value_at(
                        self.read_parameter(opcode.parameters[0]) as usize,
                        input.trim().parse::<Int>().unwrap(),
                    );
                }
                Output => {
                    print!("{}", self.read_parameter(opcode.parameters[0]) as usize);
                }
                Terminate => running = false,
            }
            self.current_idx += opcode.variant.instruction_len();
        }
        Ok(())
    }
    pub fn locate_target(&mut self, target: Int) -> Result<(Int, Int), io::Error> {
        for noun in 0..=MAX_INPUT {
            for verb in 0..=MAX_INPUT {
                self.reset();
                self.enter_inputs(noun, verb);
                self.execute()?;
                if self.result() == target {
                    return Ok((noun, verb));
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
    fn get_opcode(&self) -> Result<Opcode, io::Error> {
        // Get variant
        let mut opcode_int = self.tape[self.current_idx];
        let variant = OpcodeVariant::new(opcode_int)?;
        let code_len = variant.instruction_len();

        // Get parameters with modes
        // Variant is ones place and tens place
        opcode_int = (opcode_int as f64 / 100.0).floor() as Int;
        // Remaining place values dictate parameter modes
        let mut parameters = Vec::new();
        for i in 1..code_len {
            parameters.push(Parameter::new(
                self.tape[self.current_idx + i],
                opcode_int % 10,
            )?);
            opcode_int = (opcode_int as f64 / 10.0).floor() as Int;
        }
        Ok(Opcode::new(variant, parameters))
    }
    fn get_value_at(&self, pos: usize) -> Int {
        self.tape[pos]
    }
    fn read_parameter(&self, p: Parameter) -> Int {
        use ParameterMode::*;
        match p.mode {
            Position => self.get_value_at(p.value as usize),
            Immediate => p.value,
        }
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
    computer.execute().unwrap();
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
        assert_eq!(intcode("1002,4,3,4,33", false).1, "1002,4,3,4,99");
    }
}
