use std::fs::File;
use std::io::{BufRead, BufReader};

static YOU_KNOW_SECOND_COLUMN: bool = true;

pub fn run_day2() {
    let file = File::open("inp/day2/input.txt")
        .expect("File not found.");
    let reader = BufReader::new(file);

    let mut total_score = 0;
    if YOU_KNOW_SECOND_COLUMN {
        for line in reader.lines() {
            let this_match = match &line.unwrap()[..] {
                "A X" => 3+0,
                "A Y" => 1+3,
                "A Z" => 2+6,
                "B X" => 1+0,
                "B Y" => 2+3,
                "B Z" => 3+6,
                "C X" => 2+0,
                "C Y" => 3+3,
                "C Z" => 1+6,
                _ => 0,
            };
            total_score += this_match;
        }
    } else {
        for line in reader.lines() {
            let this_match = match &line.unwrap()[..] {
                "A X" => 1+3,
                "A Y" => 2+6,
                "A Z" => 3+0,
                "B X" => 1+0,
                "B Y" => 2+3,
                "B Z" => 3+6,
                "C X" => 1+6,
                "C Y" => 2+0,
                "C Z" => 3+3,
                _ => 0,
            };
            total_score += this_match;
        }
    }
    println!("The total score is {total_score:?}");
}
