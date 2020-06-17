//! This module implements an IntCode interpreter.

use std::convert::TryFrom;

// The following terminology notes are taken from day 2 part 2
//  - memory: the list of integers used when interpreting
//  - address/position: the value at a given index into memory
//  - opcode: mark the beginning of an instruction and denote the instruction
//  - parameters: the values after an instruction used by the instruction
//  - instruction pointer: the address of the current instruction

#[derive(Debug, PartialEq)]
enum OpCode {
    Add = 1,        // *(pc+1) + *(pc+2) => *(pc+3)
    Multiply = 2,   // *(pc+1) * *(pc+2) => *(pc+3)
    ReadIn = 3,     // store input to *(pc+1)
    WriteOut = 4,   // print value of *(pc+1) to output
    JmpIfTrue = 5,  // jump if *(pc+1) != 0 => ip = *(pc+2)
    JmpIfFalse = 6, // jump if *(pc+1) == 0 => ip = *(pc+2)
    LessThan = 7,   // if *(pc+1) < *(pc+2) => *(pc+3) = 1, else 0
    Equals = 8,     // if *(pc+1) == *(pc+2) => *(pc+3) = 1, else 0
    Halt = 99,
}

impl TryFrom<isize> for OpCode {
    type Error = &'static str;

    fn try_from(num: isize) -> Result<Self, Self::Error> {
        match num {
            1 => Ok(Self::Add),
            2 => Ok(Self::Multiply),
            3 => Ok(Self::ReadIn),
            4 => Ok(Self::WriteOut),
            5 => Ok(Self::JmpIfTrue),
            6 => Ok(Self::JmpIfFalse),
            7 => Ok(Self::LessThan),
            8 => Ok(Self::Equals),

            99 => Ok(Self::Halt),

            _ => Err("invalid opcode value"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum AddrMode {
    Pos = 0,
    Imm = 1,
}

impl TryFrom<isize> for AddrMode {
    type Error = &'static str;

    fn try_from(num: isize) -> Result<Self, Self::Error> {
        match num {
            0 => Ok(Self::Pos),
            1 => Ok(Self::Imm),
            _ => Err("invalid address mode value"),
        }
    }
}

#[derive(Debug)]
enum IPChange {
    Delta(isize),
    New(usize),
    Halt,
}

/// Parse instruction will take a full instruction, and split it into the original instruction
/// along with addressing modes for each argument.
fn parse_instruction(word: isize) -> Result<(OpCode, AddrMode, AddrMode, AddrMode), &'static str> {
    if word <= 0 {
        return Err("instruction word must be greater than zero");
    }

    Ok((
        OpCode::try_from(word % 100)?,          // first two digits are op
        AddrMode::try_from(word / 100 % 10)?,   // 100s place
        AddrMode::try_from(word / 1000 % 10)?,  // 1000s place
        AddrMode::try_from(word / 10000 % 10)?, // 10000s place
    ))
}

/// Trait is used by interpret for reading information interactively
pub trait Input {
    fn get_isize(&mut self) -> isize;
}

/// Trait is used by `interpret` for writing information interactively
pub trait Output {
    fn write_isize(&mut self, val: isize) -> ();
}

// Implementations for Input trait

impl Input for () {
    fn get_isize(&mut self) -> isize {
        panic!("Program requested input, but input source was ()");
    }
}

impl Input for isize {
    fn get_isize(&mut self) -> isize {
        *self
    }
}

// Implementations for Output trait

impl Output for () {
    fn write_isize(&mut self, _val: isize) -> () {
        panic!("Program attempted to write value, but out was ()");
    }
}

impl Output for &mut Vec<isize> {
    fn write_isize(&mut self, val: isize) -> () {
        self.push(val)
    }
}

/// Interpret array as an IntCode program.
///
/// `mem` is the initial machine memory state, it is modified during the run
///
/// Will panic if it encounters an unknown opcode
pub fn interpret(mut mem: &mut [isize], mut input: impl Input, mut output: impl Output) -> isize {
    let mut ip: usize = 0;
    loop {
        match step(&mut mem, ip, &mut input, &mut output) {
            IPChange::Delta(delta) => ip = (ip as isize + delta) as usize,
            IPChange::New(new) => ip = new,
            IPChange::Halt => break,
        }
    }

    mem[0]
}

fn step(
    mem: &mut [isize],
    ip: usize,
    input: &mut impl Input,
    output: &mut impl Output,
) -> IPChange {
    use AddrMode::*;
    use OpCode::*;

    let (op, addr1, addr2, addr3) = match parse_instruction(mem[ip]) {
        Ok(val) => val,
        Err(err) => {
            println!(
                "State:\n\tIP: {}\n\tVals: {:?}, {:?}, {:?}, {:?}",
                ip,
                mem.get(ip),
                mem.get(ip + 1),
                mem.get(ip + 2),
                mem.get(ip + 3)
            );
            panic!(format!("Encountered unrecoverable error: {}", err));
        }
    };
    // placing Halt check here so that args can be extracted without duplicating their code all
    // over the place
    if op == Halt {
        return IPChange::Halt;
    }

    // HACK: this whole block is a hack, need to wrap memory up in a new type and provide accessors
    // that understand addressing modes
    let arg1 = match addr1 {
        Imm => mem.get(ip + 1),
        Pos => mem.get(*mem.get(ip + 1).unwrap_or(&0) as usize),
    }
    .unwrap_or(&-1337);
    let arg2 = match addr2 {
        Imm => mem.get(ip + 2),
        Pos => mem.get(*mem.get(ip + 2).unwrap_or(&0) as usize),
    }
    .unwrap_or(&-1337);

    match op {
        Add => {
            mem[mem[ip + 3] as usize] = arg1 + arg2;
            IPChange::Delta(4)
        }
        Multiply => {
            mem[mem[ip + 3] as usize] = arg1 * arg2;
            IPChange::Delta(4)
        }
        ReadIn => {
            mem[mem[ip + 1] as usize] = input.get_isize();
            IPChange::Delta(2)
        }
        WriteOut => {
            output.write_isize(mem[mem[ip + 1] as usize]);
            IPChange::Delta(2)
        }
        JmpIfTrue => {
            if *arg1 != 0 {
                IPChange::New(usize::try_from(*arg2).unwrap())
            } else {
                IPChange::Delta(3)
            }
        }
        JmpIfFalse => {
            if *arg1 == 0 {
                IPChange::New(usize::try_from(*arg2).unwrap())
            } else {
                IPChange::Delta(3)
            }
        }
        LessThan => {
            mem[mem[ip + 3] as usize] = if arg1 < arg2 { 1 } else { 0 };
            IPChange::Delta(4)
        }
        Equals => {
            mem[mem[ip + 3] as usize] = if arg1 == arg2 { 1 } else { 0 };
            IPChange::Delta(4)
        }
        Halt => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn interpret_day2_examples() {
        let mut programs = vec![
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![1, 0, 0, 0, 99],
            vec![2, 3, 0, 3, 99],
            vec![2, 4, 4, 5, 99, 0],
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
        ];
        let outputs = vec![
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![2, 0, 0, 0, 99],
            vec![2, 3, 0, 6, 99],
            vec![2, 4, 4, 5, 99, 9801],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        ];
        for i in 0..programs.len() {
            assert_eq!(interpret(&mut programs[i], (), ()), outputs[i][0]);
            assert_eq!(programs[i], outputs[i]);
        }
    }

    #[test]
    fn test_parse_instruction() {
        use AddrMode::*;
        use OpCode::*;

        type Output = (OpCode, AddrMode, AddrMode, AddrMode);
        fn eq(left: Output, right: Output) -> bool {
            left.0 == right.0 && left.1 == right.1 && left.2 == right.2 && left.3 == right.3
        }

        // from day 5 examples
        assert!(eq(
            parse_instruction(1002).unwrap(),
            (Multiply, Pos, Imm, Pos)
        ));

        // synthetic
        assert!(eq(parse_instruction(2).unwrap(), (Multiply, Pos, Pos, Pos)));
        assert!(eq(parse_instruction(11101).unwrap(), (Add, Imm, Imm, Imm)));
        assert!(eq(parse_instruction(10101).unwrap(), (Add, Imm, Pos, Imm)));
        assert!(eq(
            parse_instruction(104).unwrap(),
            (WriteOut, Imm, Pos, Pos)
        ));
        assert!(eq(
            parse_instruction(10003).unwrap(),
            (ReadIn, Pos, Pos, Imm)
        ));
    }

    #[test]
    fn day5_snippets() {
        // This tests immediate and positional addressing and negative immediate support
        // Should: find (100 + -1), store result @4
        let mut simple_prog = vec![1101, 100, -1, 4, 0];
        interpret(&mut simple_prog, (), ());
        assert_eq!(simple_prog[4], 99);

        // This should save whatever it gets from input to @0, then print it back out
        let arb_input = 10346;
        let mut output = Vec::new();
        let mut simple_io = vec![3, 0, 4, 0, 99];
        interpret(&mut simple_io, arb_input, &mut output);
        println!("{:?}", output[0]);
        println!("{:?}", simple_io);
        assert_eq!(simple_io[0], arb_input);
        assert_eq!(output[0], arb_input);
    }

    #[test]
    fn day5_jump_tests() {
        // These programs compare the input to 8, outputting 1 if eq or lt, 0 otherwise
        // they use different methods for each
        // test eq
        let progs_eq_to_eight = vec![
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], // positional
            vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],     // immediate
        ];
        for (input, exp_out) in vec![(0, 0), (8, 1), (-8, 0), (10, 0)] {
            for i in 0..progs_eq_to_eight.len() {
                let mut prog = progs_eq_to_eight[i].clone();
                let mut output = Vec::new();
                interpret(&mut prog, input, &mut output);
                assert_eq!(exp_out, output[0]);
            }
        }
        // test lt
        let progs_lt_eight = vec![
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], // lt positional
            vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],     // lt immediate
        ];
        for (input, exp_out) in vec![(0, 1), (-1, 1), (8, 0), (10, 0)] {
            for i in 0..progs_lt_eight.len() {
                let mut prog = progs_lt_eight[i].clone();
                let mut output = Vec::new();
                interpret(&mut prog, input, &mut output);
                assert_eq!(exp_out, output[0], "input: {}", input);
            }
        }
        // test jump
        let jump_progs = vec![
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], // positional
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],         // immediate
        ];
        for (input, exp_out) in vec![(0, 0), (-1, 1), (8, 1), (10, 1)] {
            for i in 0..jump_progs.len() {
                let mut prog = jump_progs[i].clone();
                let mut output = Vec::new();
                interpret(&mut prog, input, &mut output);
                assert_eq!(exp_out, output[0], "input: {}", input);
            }
        }
    }

    #[test]
    fn day5_large_test() {
        let jmp_prog = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let in_outs = vec![
            (-1, 999),
            (0, 999),
            (5, 999),
            (8, 1000),
            (9, 1001),
            (14240, 1001),
        ];
        let mut output = Vec::new();
        for (input, exp_out) in in_outs.into_iter() {
            println!("{:?}", input);
            let mut prog = jmp_prog.clone();
            output.clear();
            interpret(&mut prog, input, &mut output);
            assert_eq!(exp_out, output[0], "input: {}", input);
        }
    }
}
