pub fn work(lines: Vec<String>) {
    println!("Part 1: {}", do_work(&lines));
}

fn do_work<T: Deref<Target = str>>(lines: &[T]) -> usize {
}

#[cfg(test)]
mod test {
    use super::*;
}
