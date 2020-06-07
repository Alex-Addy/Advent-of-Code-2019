use std::io;

use std::io::{BufRead, BufReader};

mod solutions;
mod utilities;

/// Print program usage to stdout
fn print_usage() {
    println!("Expected usage:");
    println!("  $> <executable> <day num> <input file>");
}

fn get_file_input(path: &str) -> io::Result<Vec<String>> {
    let file = std::fs::File::open(path)?;
    BufReader::new(file).lines().collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        print_usage();
        return;
    }
    let day: u8 = match args[0].parse() {
        Ok(x) => x,
        Err(err) => {
            println!("Error while parsing <day>: {:?}", err);
            return;
        }
    };
    let input = match get_file_input(&args[1]) {
        Ok(x) => x,
        Err(err) => {
            println!("Error while getting input: {:?}", err);
            return;
        }
    };

    match day {
        1 => solutions::day01::work(&input),
        2 => solutions::day02::work(&input),
        3 => solutions::day03::work(&input),
        4 => solutions::day04::work(&input),
        5 => solutions::day05::work(&input),
        6 => unimplemented!(),
        7 => unimplemented!(),
        8 => unimplemented!(),
        9 => unimplemented!(),
        10 => unimplemented!(),
        11 => unimplemented!(),
        12 => unimplemented!(),
        13 => unimplemented!(),
        14 => unimplemented!(),
        15 => unimplemented!(),
        16 => unimplemented!(),
        17 => unimplemented!(),
        18 => unimplemented!(),
        19 => unimplemented!(),
        20 => unimplemented!(),
        21 => unimplemented!(),
        22 => unimplemented!(),
        23 => unimplemented!(),
        24 => unimplemented!(),
        25 => unimplemented!(),
        _ => {
            print_usage();
            return;
        }
    }
}
