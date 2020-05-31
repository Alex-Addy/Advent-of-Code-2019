//! This module implements an IntCode interpreter.

// The following terminology notes are taken from day 2 part 2
//  - memory: the list of integers used when interpreting
//  - address/position: the value at a given index into memory
//  - opcode: mark the beginning of an instruction and denote the instruction
//  - parameters: the values after an instruction used by the instruction
//  - instruction pointer: the address of the current instruction

const ADD: usize = 1; // *(pc+1) + *(pc+2) => *(pc+3)
const MULTIPLY: usize = 2; // *(pc+1) * *(pc+2) => *(pc+3)
const SAVE: usize = 3; // store input to *(pc+1)
const PRINT: usize = 4; // print value of *(pc+1) to output
const HALT: usize = 99;

#[derive(Debug, PartialEq)]
enum AddrMode {
    Pos = 0,
    Imm = 1,
}

impl From<usize> for AddrMode {
    fn from(num: usize) -> Self {
        match num {
            0 => Self::Pos,
            1 => Self::Imm,
            _ => panic!(format!("Got invalid address mode value {}", num)),
        }
    }
}

/// Parse instruction will take a full instruction, and split it into the original instruction
/// along with addressing modes for each argument.
fn parse_instruction(word: usize) -> (usize, AddrMode, AddrMode, AddrMode) {
    let cmd = word % 100; // first two digits are the instruction
    (
        cmd,
        AddrMode::from(word / 100 % 10),
        AddrMode::from(word / 1000 % 10),
        AddrMode::from(word / 10000 % 10),
    )
}

/// Interpret array as an IntCode program.
///
/// `mem` is the initial machine memory state, it is modified during the run
///
/// Will panic if it encounters an unknown opcode
pub fn interpret(mem: &mut [usize]) -> usize {
    let mut ip = 0;
    while mem[ip] != HALT {
        match mem[ip] {
            ADD => {
                mem[mem[ip + 3]] = mem[mem[ip + 1]] + mem[mem[ip + 2]];
                ip += 4;
            }
            MULTIPLY => {
                mem[mem[ip + 3]] = mem[mem[ip + 1]] * mem[mem[ip + 2]];
                ip += 4;
            }
            _ => {
                panic!("Unexpected opcode: {} at {}", mem[ip], ip);
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
            assert_eq!(interpret(&mut programs[i]), outputs[i][0]);
            assert_eq!(programs[i], outputs[i]);
        }
    }

    #[test]
    fn test_parse_instruction() {
        use AddrMode::*;

        type Output = (usize, AddrMode, AddrMode, AddrMode);
        fn eq(left: Output, right: Output) -> bool {
            left.0 == right.0 && left.1 == right.1 && left.2 == right.2 && left.3 == right.3
        }

        // from day 5 examples
        assert!(eq(parse_instruction(1002), (2, Pos, Imm, Pos)));

        // synthetic
        assert!(eq(parse_instruction(2), (2, Pos, Pos, Pos)));
        assert!(eq(parse_instruction(11104), (4, Imm, Imm, Imm)));
        assert!(eq(parse_instruction(10113), (13, Imm, Pos, Imm)));
        assert!(eq(parse_instruction(104), (4, Imm, Pos, Pos)));
        assert!(eq(parse_instruction(10004), (4, Pos, Pos, Imm)));
    }
}
