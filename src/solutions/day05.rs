use crate::utilities::intcode::interpret;

pub fn work(lines: &Vec<String>) {
    println!("{:#?}", do_work(&lines[0]));
}

fn do_work(line: &str) -> Vec<isize> {
    let mut program: Vec<isize> = line
        .split(',')
        .map(|piece| piece.parse().unwrap())
        .collect();
    let mut output = Vec::new();
    interpret(&mut program, 1, &mut output);
    output
}

#[cfg(test)]
mod test {
    use super::*;
    fn part_1_test() {
        let input = include_str!("../../inputs/day05.txt");
        let exp_out = vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 8332629];
        let output = do_work(input);
        assert_eq!(output, exp_out);
    }
}
