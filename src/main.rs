
/// Print program usage to stdout
fn print_usage() {
    println!("Expected usage:");
    println!("  $> <executable> <day num> <input file>");
    println!("Where <day> is a value in [1, 25], and"
             " <input file> is the text file input for the problem");
}

fn main() {
    let args = std::env::args().skip(1).collect();
    if len(args) != 2 {
        print_usage();
        return;
    }
    let day = args[0].parse::<u8>()?;
    let input_file = args[1];

    match day {
        1 => unimplemented!(),
        2 => unimplemented!(),
        3 => unimplemented!(),
        4 => unimplemented!(),
        5 => unimplemented!(),
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
        },
    }
}
