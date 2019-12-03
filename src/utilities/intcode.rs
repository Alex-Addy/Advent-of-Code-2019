//! This module implements an IntCode interpreter.

// The following terminology notes are taken from day 2 part 2
//  - memory: the list of integers used when interpreting
//  - address/position: the value at a given index into memory
//  - opcode: mark the beginning of an instruction and denote the instruction
//  - parameters: the values after an instruction used by the instruction
//  - instruction pointer: the address of the current instruction

const ADD: usize = 1; // *(pc+1) + *(pc+2) => *(pc+3)
const MULTIPLY: usize = 2; // *(pc+1) * *(pc+2) => *(pc+3)
const HALT: usize = 99;

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
                mem[mem[ip+3]] = mem[mem[ip+1]] + mem[mem[ip+2]];
                ip += 4;
            }
            MULTIPLY => {
                mem[mem[ip+3]] = mem[mem[ip+1]] * mem[mem[ip+2]];
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
            vec![1,9,10,3,2,3,11,0,99,30,40,50],
            vec![1,0,0,0,99],
            vec![2,3,0,3,99],
            vec![2,4,4,5,99,0],
            vec![1,1,1,4,99,5,6,0,99],
        ];
        let outputs = vec![
            vec![3500,9,10,70,2,3,11,0,99,30,40,50],
            vec![2,0,0,0,99],
            vec![2,3,0,6,99],
            vec![2,4,4,5,99,9801],
            vec![30,1,1,4,2,5,6,0,99],
        ];
        for i in 0..programs.len() {
            assert_eq!(interpret(&mut programs[i]), outputs[i][0]);
            assert_eq!(programs[i], outputs[i]);
        }
    }
}
