pub fn work(_lines: &Vec<String>) {
    let lower = 264360;
    let upper = 746325;
    println!("Part 1: {}", find_pass(lower, upper));
}

fn find_pass(lower: usize, upper: usize) -> usize {
    assert!(lower < upper);
    let mut cur_pass = lower;
    let mut count = 0;
    while cur_pass <= upper {
        if is_pass(cur_pass) {
            count += 1;
        }
        cur_pass += 1;
    }
    count
}

fn is_pass(pass_num: usize) -> bool {
    let pass = format!("{}", pass_num);
    let adj_dig = pass.as_bytes().windows(2).any(|w| w[0] == w[1]);
    let alw_inc = pass.as_bytes().windows(2).all(|w| w[0] <= w[1]);
    pass.len() == 6 && adj_dig && alw_inc
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pass_example_check() {
        assert_eq!(is_pass(111111), true);
        assert_eq!(is_pass(223450), false);
        assert_eq!(is_pass(123789), false);
    }
}
