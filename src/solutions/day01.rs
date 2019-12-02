pub fn work(lines: &Vec<String>) {
    println!("Part 1: {}", do_work(&lines));
}

fn do_work(lines: &Vec<String>) -> usize {
    lines.iter()
        .map(|line| mass_to_fuel(line.parse().unwrap()))
        .sum()
}

fn mass_to_fuel(mass: usize) -> usize {
    mass / 3 - 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mass_to_fuel_examples() {
        assert_eq!(mass_to_fuel(12), 2);
        assert_eq!(mass_to_fuel(14), 2);
        assert_eq!(mass_to_fuel(1969), 654);
        assert_eq!(mass_to_fuel(100756), 33583);
    }
}
