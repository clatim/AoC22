use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_day4(part: u8) {
    let reader = BufReader::new(File::open("inp/day4/input.txt").expect("File not found."));

    let mut sum = 0;

    for line in reader.lines() {
        let (elf1, elf2) = line
            .unwrap()
            .split(",")
            .map(|s| s.to_string())
            .collect_tuple()
            .unwrap();
        let range1 = extract_start_end(elf1);
        let range2 = extract_start_end(elf2);
        if part == 2 {
            if any_overlap(range1, range2) || any_overlap(range2, range1) {
                sum += 1;
            }
        } else if is_subset(range1, range2) || is_subset(range2, range1) {
            sum += 1;
        }
    }

    println!("The number of subsets is {sum}");
}

fn extract_start_end(to_clean: String) -> (i32, i32) {
    to_clean
        .split("-")
        .map(|n| n.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn is_subset(original: (i32, i32), check: (i32, i32)) -> bool {
    check.0 >= original.0 && check.1 <= original.1
}

fn any_overlap(original: (i32, i32), check: (i32, i32)) -> bool {
    check.0 >= original.0 && check.0 <= original.1 || check.1 <= original.1 && check.1 >= original.0
}
