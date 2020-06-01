//! This module implements an IntCode interpreter.

// The following terminology notes are taken from day 2 part 2
//  - memory: the list of integers used when interpreting
//  - address/position: the value at a given index into memory
//  - opcode: mark the beginning of an instruction and denote the instruction
//  - parameters: the values after an instruction used by the instruction
//  - instruction pointer: the address of the current instruction

#[derive(Debug, PartialEq)]
enum OpCode {
    Add = 1,      // *(pc+1) + *(pc+2) => *(pc+3)
    Multiply = 2, // *(pc+1) * *(pc+2) => *(pc+3)
    ReadIn = 3,   // store input to *(pc+1)
    WriteOut = 4, // print value of *(pc+1) to output
    Halt = 99,
}

impl From<isize> for OpCode {
    fn from(num: isize) -> Self {
        match num {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::ReadIn,
            4 => Self::WriteOut,

            99 => Self::Halt,

            _ => panic!(format!("Got invalid opcode: {}", num)),
        }
    }
}

#[derive(Debug, PartialEq)]
enum AddrMode {
    Pos = 0,
    Imm = 1,
}

impl From<isize> for AddrMode {
    fn from(num: isize) -> Self {
        match num {
            0 => Self::Pos,
            1 => Self::Imm,
            _ => panic!(format!("Got invalid address mode value {}", num)),
        }
    }
}

/// Parse instruction will take a full instruction, and split it into the original instruction
/// along with addressing modes for each argument.
fn parse_instruction(word: isize) -> (OpCode, AddrMode, AddrMode, AddrMode) {
    assert!(word > 0);

    (
        OpCode::from(word % 100),          // first two digits are op
        AddrMode::from(word / 100 % 10),   // 100s place
        AddrMode::from(word / 1000 % 10),  // 1000s place
        AddrMode::from(word / 10000 % 10), // 10000s place
    )
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

impl Input for dyn Iterator<Item = isize> {
    fn get_isize(&mut self) -> isize {
        self.next()
            .expect("Program requested input, but Iterator return None")
    }
}

// Implementations for Output trait

impl Output for () {
    fn write_isize(&mut self, _val: isize) -> () {
        panic!("Program attempted to write value, but out was ()");
    }
}

impl Output for Vec<isize> {
    fn write_isize(&mut self, val: isize) -> () {
        self.push(val)
    }
}

/// Interpret array as an IntCode program.
///
/// `mem` is the initial machine memory state, it is modified during the run
///
/// Will panic if it encounters an unknown opcode
pub fn interpret(mem: &mut [isize], mut input: impl Input, mut output: impl Output) -> isize {
    use OpCode::*;

    let mut ip = 0;
    loop {
        match OpCode::from(mem[ip]) {
            Add => {
                mem[mem[ip + 3] as usize] = mem[mem[ip + 1] as usize] + mem[mem[ip + 2] as usize];
                ip += 4;
            }
            Multiply => {
                mem[mem[ip + 3] as usize] = mem[mem[ip + 1] as usize] * mem[mem[ip + 2] as usize];
                ip += 4;
            }
            ReadIn => {
                mem[mem[ip + 1] as usize] = input.get_isize();
                ip += 2;
            }
            WriteOut => {
                output.write_isize(mem[mem[ip + 1] as usize]);
                ip += 2;
            }
            Halt => {
                break;
            }
        }
    }

    mem[0]
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
        assert!(eq(parse_instruction(1002), (Multiply, Pos, Imm, Pos)));

        // synthetic
        assert!(eq(parse_instruction(2), (Multiply, Pos, Pos, Pos)));
        assert!(eq(parse_instruction(11101), (Add, Imm, Imm, Imm)));
        assert!(eq(parse_instruction(10101), (Add, Imm, Pos, Imm)));
        assert!(eq(parse_instruction(104), (WriteOut, Imm, Pos, Pos)));
        assert!(eq(parse_instruction(10003), (ReadIn, Pos, Pos, Imm)));
    }
}
