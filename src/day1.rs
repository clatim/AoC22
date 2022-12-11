use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

static SINGLE_ELF: bool = false;

pub fn run_day1() {

    let file = File::open("inp/day1/input.txt")
        .expect("File not found.");
    let reader = BufReader::new(file);

    let mut current_elf: i32 = 0;
    let mut all_elves: Vec<i32> = Vec::new();
    for line in reader.lines() {
        match line.unwrap().parse::<i32>() {
            Ok(n) => current_elf += n,
            Err(_) => {
                all_elves.push(current_elf);
                current_elf = 0;
            },
        };
    }
    // Don't forget about the last elf!
    all_elves.push(current_elf);
    println!("The elves are carrying all these calories! {all_elves:?}");

    if SINGLE_ELF {
        // Now find the maximum elf
        let big_elf = all_elves
            .iter()
            .max()
            .unwrap();
        println!("The biggest elf is {big_elf:?}");
    } else {
        let mut  total_calories = 0;
        for _ in 1..=3 {
            let this_elf = all_elves
                .iter()
                .enumerate()
                .max_by_key(|(_, &value)| value)
                .map(|(index, _)| index)
                .unwrap();
            println!("He big {this_elf}");
            total_calories += all_elves.remove(this_elf as usize);
        }
        println!("The total calores are {total_calories}");
    }

}

