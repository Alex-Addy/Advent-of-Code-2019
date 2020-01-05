pub fn work(_lines: &Vec<String>) {
    let lower = 264360;
    let upper = 746325;
    println!("Part 1: {}", find_pass(lower, upper));
    println!("Part 2: {}", find_pass_2(lower, upper));
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

fn find_pass_2(lower: usize, upper: usize) -> usize {
    assert!(lower < upper);
    let mut cur_pass = lower;
    let mut count = 0;
    while cur_pass <= upper {
        if is_pass_2(cur_pass) {
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

fn is_pass_2(pass_num: usize) -> bool {
    let pass = format!("{}", pass_num);
    let pass = pass.as_bytes();

    let alw_inc = pass.windows(2).all(|w| w[0] <= w[1]);
    if (pass.len() == 6 && alw_inc) == false {
        return false;
    }

    for i in 0..(pass.len()-1) {
        if pass[i] != pass[i+1] {
            continue;
        }
        if i > 1 {
            if pass[i-1] == pass[i] {
                continue;
            }
        }
        if let Some(val) = pass.get(i+2) {
            if *val == pass[i] {
                continue;
            }
        }
        return true;
    }
    return false;
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

    #[test]
    fn pass_2_example_check() {
        assert_eq!(is_pass_2(112233), true);
        assert_eq!(is_pass_2(123444), false);
        assert_eq!(is_pass_2(111122), true);
    }
}
