use clap::Parser;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

#[derive(Parser)]
struct Cli {
    // The day you want to run
    day: u8,
    // The part to run
    // TODO: make this an enum
    part: u8,
}

fn main() {
    let cli = Cli::parse();
    println!("Doing day {:?} part {:?}", cli.day, cli.part);
    let part = cli.part;
    assert!(
        part == 1 || part == 2,
        "Second command line argument (part) must be 1 or 2"
    );

    match cli.day {
        1 => day1::run_day1(part),
        2 => day2::run_day2(part),
        3 => day3::run_day3(part),
        4 => day4::run_day4(part),
        5 => day5::run_day5(part),
        6 => {
            let input = include_str!("../inp/day6/input.txt");
            let _ = day6::run_day6(input, part);
        }
        _ => println!("I haven't done day {} yet.", cli.day),
    }
}
