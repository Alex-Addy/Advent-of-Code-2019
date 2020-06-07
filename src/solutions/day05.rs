use crate::utilities::intcode::interpret;

pub fn work(lines: &Vec<String>) {
    do_work(&lines);
}

fn do_work(lines: &Vec<String>) -> () {
    let mut program: Vec<isize> = lines[0]
        .split(',')
        .map(|piece| piece.parse().unwrap())
        .collect();
    let mut output = Vec::new();
    interpret(&mut program, 1, &mut output);
    println!("{:#?}", output);
}

