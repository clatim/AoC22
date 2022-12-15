use std::env;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "You must pass in the day and the part on the command line.");
    let day: u8 = args[1]
        .parse()
        .expect("First command line argument (day) must be an integer.");
    let part: u8 = args[2]
        .parse()
        .expect("Second command line argument (part) must be an integer.");
    assert!(part == 1 || part == 2, "Second command line argument (part) must be 1 or 2");
    
    match day {
        1 => day1::run_day1(),
        2 => day2::run_day2(),
        3 => day3::run_day3(),
        4 => day4::run_day4(),
        5 => day5::run_day5(),
        6 => {
            let input = include_str!("../inp/day6/input.txt");
            let _= day6::run_day6(input, part);
        },
        _ => println!("I haven't done day {day} yet."),
    }
}
