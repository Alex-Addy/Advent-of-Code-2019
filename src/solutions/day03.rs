use std::collections::HashSet;

pub fn work(lines: &Vec<String>) {
    println!("Part 1: {}", do_work(&lines));
}

fn do_work(lines: &Vec<String>) -> usize {
    assert_eq!(lines.len(), 2);
    let wire_1_positions = draw_wire(&lines[0]);
    let wire_2_positions = draw_wire(&lines[1]);

    let mut min_dist = std::usize::MAX;
    for pos in wire_1_positions.iter() {
        if wire_2_positions.contains(pos) {
            let dist = (pos.0.abs() + pos.1.abs()) as usize;
            if dist < min_dist {
                min_dist = dist;
            }
        }
    }

    min_dist
}

enum Move {
    XDelta(isize), // Right is +, Left is -
    YDelta(isize), // Up is +, Down is -
}

fn parse_move(mv: &str) -> Move {
    let mut chars = mv.chars();
    let dir = chars.next().unwrap();
    let val = chars.collect::<String>().parse().unwrap();
    match dir {
        'R' => Move::XDelta(val),
        'L' => Move::XDelta(-val),
        'U' => Move::YDelta(val),
        'D' => Move::YDelta(-val),
        _ => panic!("Got unexpected direction: {}", dir),
    }
}

fn draw_wire(line: &str) -> HashSet<(isize, isize)> {
    let mut x = 0; // Left, Right
    let mut y = 0; // Up, Down
    let mut positions = HashSet::new();
    for step in line.split(',') {
        match parse_move(step) {
            Move::XDelta(d_x) => {
                let step = if d_x > 0 { 1} else { -1 };
                for _ in 0..d_x.abs() {
                    x += step;
                    positions.insert((x,y));
                }
            }
            Move::YDelta(d_y) => {
                let step = if d_y > 0 { 1} else { -1 };
                for _ in 0..d_y.abs() {
                    y += step;
                    positions.insert((x,y));
                }
            }
        }
    }

    positions
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_examples() {
        let examples = vec![
            vec!["R8,U5,L5,D3".to_owned(), "U7,R6,D4,L4".to_owned()],
            vec!["R75,D30,R83,U83,L12,D49,R71,U7,L72".to_owned(),
                "U62,R66,U55,R34,D71,R55,D58,R83".to_owned()],
            vec!["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_owned(),
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_owned()],
        ];
        let expected = vec![6, 159, 135];

        for i in 0..examples.len() {
            assert_eq!(do_work(&examples[i]), expected[i]);
        }
    }
}
