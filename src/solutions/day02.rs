use crate::utilities::intcode::interpret;

pub fn work(lines: &Vec<String>) {
    println!("Part 1: {}", do_work(&lines));
    println!("Part 2: {}", do_work_2(&lines));
}

fn do_work(lines: &Vec<String>) -> isize {
    let mut program: Vec<isize> = lines[0]
        .split(',')
        .map(|piece| piece.parse().unwrap())
        .collect();
    // set program up for 1202 program
    program[1] = 12;
    program[2] = 2;
    interpret(&mut program, (), ())
}

fn do_work_2(lines: &Vec<String>) -> isize {
    let program: Vec<isize> = lines[0]
        .split(',')
        .map(|piece| piece.parse().unwrap())
        .collect();
    for i in 0..99 {
        for k in 0..99 {
            let mut tmp_mem = program.clone();
            tmp_mem[1] = i; // noun
            tmp_mem[2] = k; // verb
            let res = interpret(&mut tmp_mem, (), ());
            if res == 19690720 {
                return i * 100 + k;
            }
        }
    }
    panic!("Couldn't find inputs!");
}

#[cfg(test)]
mod test {}
