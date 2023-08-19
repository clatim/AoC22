use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_day3(part: u8) {
    let file = File::open("inp/day3/input.txt").expect("File not found.");
    let reader = BufReader::new(file);

    let mut sum = 0;

    if part == 2 {
        for (elf1, elf2, elf3) in reader.lines().tuples() {
            let shared_1_and_2 = &shared_items(&elf1.unwrap(), &elf2.unwrap())
                .iter()
                .collect::<String>()[..];
            let all_shared = shared_items(&shared_1_and_2, &elf3.unwrap())[0];
            let value = convert_to_priority(all_shared);
            sum += value;
        }
    } else {
        for line in reader.lines() {
            let contents = line.unwrap();
            let length = contents.chars().count();
            // Make sure that the backpack contains an even number of things.
            assert!(length % 2 == 0);

            let first_comp = &contents[..length / 2];
            let second_comp = &contents[length / 2..];

            // Sometimes the same thing can be in the bag multiple times.
            // I only want to know WHAT matches, I don't care how many times
            // it appears.
            // Therefore, just get the first entry of the Vec of characters that match.
            let contains = shared_items(first_comp, second_comp)[0];
            let value = convert_to_priority(contains);
            sum += value;
        }
    }
    println!("The sum is {sum}");
}

fn shared_items(first: &str, second: &str) -> Vec<char> {
    // Finds characters that are shared between first and second
    second.chars().filter(|c| first.contains(*c)).collect()
}

fn convert_to_priority(item: char) -> u32 {
    // Converts a letter to its priority value.
    if item.is_lowercase() {
        return item as u32 - 96;
    } else {
        return item as u32 - 38;
    }
}
