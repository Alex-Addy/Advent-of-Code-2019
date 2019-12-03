pub fn work(lines: &Vec<String>) {
    println!("Part 1: {}", do_work(&lines));
}

fn do_work(lines: &Vec<String>) -> usize {
    let mut program: Vec<usize> = lines[0].split(',').map(|piece| piece.parse().unwrap()).collect();
    // set program up for 1202 program
    program[1] = 12;
    program[2] = 2;
    interpret(&mut program)
}

fn interpret(mem: &mut [usize]) -> usize {
    let mut pc = 0;
    while mem[pc] != 99 { // Halt
        match mem[pc] {
            1 => { // Add
                mem[mem[pc+3]] = mem[mem[pc+1]] + mem[mem[pc+2]];
                pc += 4;
            }
            2 => { // Mul
                mem[mem[pc+3]] = mem[mem[pc+1]] * mem[mem[pc+2]];
                pc += 4;
            }
            _ => {
                panic!("Unexpected opcode: {} at {}", mem[pc], pc);
            }
        }
    }

    mem[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpret_examples() {
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
