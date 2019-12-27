use std::{
    fmt,
    io::{self, ErrorKind::*, Read, Write},
};
pub type Int = isize;
const INT_SIZE: usize = std::mem::size_of::<Int>();

const MAX_INPUT: Int = 99;

#[derive(Debug, Clone, Copy, PartialEq)]
enum OpcodeVariant {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpT = 5,
    JumpF = 6,
    LessThan = 7,
    Equals = 8,
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
            5 => Ok(JumpT),
            6 => Ok(JumpF),
            7 => Ok(LessThan),
            8 => Ok(Equals),
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
            Add | Multiply | LessThan | Equals => 4,
            JumpT | JumpF => 3,
            Input | Output => 2,
            Terminate => 1,
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

pub struct IntcodeComputer {
    current_idx: usize,
    current_mode: ParameterMode,
    program: String,
    tape: Vec<Int>,
    input_stream: Box<dyn Read>,
    output_stream: Box<dyn Write>,
}

impl Default for IntcodeComputer {
    fn default() -> Self {
        Self {
            current_idx: usize::default(),
            current_mode: ParameterMode::default(),
            program: String::default(),
            tape: Vec::default(),
            // Maybe just store a Vec<Int>??
            input_stream: Box::new(io::stdin()),
            output_stream: Box::new(io::stdout()),
        }
    }
}

impl IntcodeComputer {
    pub fn new(input: &str, user_inputs: &[Int]) -> Self {
        let mut ret = Self::default();
        ret.program = input.to_string();
        ret.init_tape();
        // If pre-defined inputs were passed, convert to byte stream
        if !user_inputs.is_empty() {
            let mut stream = vec![];
            for i in user_inputs {
                let bytes = i.to_ne_bytes();
                for b in &bytes {
                    stream.push(*b);
                }
            }
            let mut input_stream = vec![];
            input_stream.copy_from_slice(stream.as_slice());
            ret.input_stream = Box::new(input_stream.as_slice());
        }
        ret
    }
    pub fn fix_1202bug(&mut self) {
        self.enter_inputs(12, 2);
    }
    pub fn execute(&mut self) -> Result<(), io::Error> {
        let mut running = true;
        while running {
            let mut hop = true;
            let opcode = self.get_opcode()?;
            use OpcodeVariant::*;
            match opcode.variant {
                Add => {
                    let lhs = self.read_parameter(opcode.parameters[0]);
                    let rhs = self.read_parameter(opcode.parameters[1]);
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
                    // If there are some left, use those, otherwise wait for stdin
                    print!("Enter value> ");
                    self.output_stream.flush()?;
                    // Read exactly one 8-byte number
                    let mut dest_buffer: [u8; INT_SIZE] = [0; INT_SIZE];
                    self.input_stream
                        .read_exact(&mut dest_buffer)
                        .expect("Unable to read destination from input stream");
                    self.set_value_at(
                        opcode.parameters[0].value as usize, // TODO destinations are weird, see line 124
                        Int::from_ne_bytes(dest_buffer),
                    );
                }
                Output => {
                    // TODO use self.output_stream
                    println!("{}", self.read_parameter(opcode.parameters[0]));
                    io::stdout().flush()?;
                }
                JumpT => {
                    let check_val = self.read_parameter(opcode.parameters[0]);
                    let jmp = self.read_parameter(opcode.parameters[1]);
                    if check_val != 0 {
                        self.current_idx = jmp as usize;
                        hop = false;
                    }
                }
                JumpF => {
                    let check_val = self.read_parameter(opcode.parameters[0]);
                    let jmp = self.read_parameter(opcode.parameters[1]);
                    if check_val == 0 {
                        self.current_idx = jmp as usize;
                        hop = false;
                    }
                }
                LessThan => {
                    let lhs = self.read_parameter(opcode.parameters[0]);
                    let rhs = self.read_parameter(opcode.parameters[1]);
                    let dest = opcode.parameters[2].value;
                    let val = if lhs < rhs { 1 } else { 0 };
                    self.set_value_at(dest as usize, val);
                }
                Equals => {
                    let lhs = self.read_parameter(opcode.parameters[0]);
                    let rhs = self.read_parameter(opcode.parameters[1]);
                    let dest = opcode.parameters[2].value;
                    let val = if lhs == rhs { 1 } else { 0 };
                    self.set_value_at(dest as usize, val);
                }
                Terminate => running = false,
            }
            if hop {
                self.current_idx += opcode.variant.instruction_len();
            }
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

pub fn intcode(input: &str, buggy: bool, user_inputs: &[Int]) -> (Int, String) {
    let mut computer = IntcodeComputer::new(input, user_inputs);
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
        assert_eq!(intcode("1,0,0,0,99", false, &[]).1, "2,0,0,0,99");
        assert_eq!(intcode("2,3,0,3,99", false, &[]).1, "2,3,0,6,99");
        assert_eq!(intcode("2,4,4,5,99,0", false, &[]).1, "2,4,4,5,99,9801");
        assert_eq!(
            intcode("1,1,1,4,99,5,6,0,99", false, &[]).1,
            "30,1,1,4,2,5,6,0,99"
        );
        assert_eq!(
            intcode("1,9,10,3,2,3,11,0,99,30,40,50", false, &[]).1,
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
    }
    #[test]
    fn test_v1_day5() {
        assert_eq!(intcode("1002,4,3,4,33", false, &[]).1, "1002,4,3,4,99");
        assert_eq!(intcode("1101,100,-1,4,0", false, &[]).1, "1101,100,-1,4,99");
    }
}
