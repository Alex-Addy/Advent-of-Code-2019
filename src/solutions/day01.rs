pub fn work(lines: &Vec<String>) {
    println!("Part 1: {}", do_work(&lines));
    println!("Part 2: {}", do_work_2(&lines));
}

fn do_work(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| mass_to_fuel(line.parse().unwrap()))
        .sum()
}

fn do_work_2(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .map(rocket_equation)
        .sum()
}

fn mass_to_fuel(mass: usize) -> usize {
    (mass / 3).saturating_sub(2)
}

fn rocket_equation(base_mass: usize) -> usize {
    let mut total_fuel = mass_to_fuel(base_mass);
    let mut last_mass = total_fuel;
    loop {
        let new_mass = mass_to_fuel(last_mass);
        if new_mass == 0 {
            break;
        }
        total_fuel += new_mass;
        last_mass = new_mass;
    }

    total_fuel
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

    #[test]
    fn test_rocket_equation_examples() {
        assert_eq!(rocket_equation(14), 2);
        assert_eq!(rocket_equation(1969), 966);
        assert_eq!(rocket_equation(100756), 50346);
    }
}
