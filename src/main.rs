use std::env;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args[1]
        .parse()
        .expect("First command line argument (day) must be an integer.");

    match day {
        1 => day1::run_day1(),
        2 => day2::run_day2(),
        3 => day3::run_day3(),
        4 => day4::run_day4(),
        5 => day5::run_day5(),
        _ => println!("I haven't done day {day} yet."),
    }
}
