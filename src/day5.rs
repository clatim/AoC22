use regex::Regex;

pub fn run_day5(part: u8) {
    let input = include_str!("../inp/day5/input.txt");
    // Finds the numbers of stacks.
    // I don't actually think this is needed.
    let has_numbers = Regex::new(r"[1-9]").unwrap();
    let mut num_stacks: u32 = 0;
    for line in input.lines() {
        if has_numbers.is_match(line) {
            num_stacks = line
                .split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .max()
                .unwrap();
            break;
        }
    }
    println!("{num_stacks}");
    let mut stack: Vec<Vec<&str>> = Vec::new();
    for _ in 0..num_stacks {
        stack.push(Vec::new());
    }

    let re = Regex::new(r" {4}|[A-Z]").unwrap();
    let numbers = Regex::new(r"[1-9]").unwrap();
    for line in input.lines() {
        if numbers.is_match(line) {
            break;
        }
        println!("line {}", line);
        for (stack_number, cap) in re.captures_iter(line).enumerate() {
            println!("{:?}", cap.get(0).unwrap().as_str().trim());
            let matched = cap.get(0).unwrap().as_str();
            match matched {
                "    " => (),
                _ => stack[stack_number].insert(0, matched),
            };
        }
    }
    println!("{:?}", stack);

    let movement = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    for line in input.lines() {
        println!("{line}");
        if !movement.is_match(line) {
            continue;
        }

        for cap in movement.captures_iter(line) {
            println!("{:?}", cap);
            let num_move = cap[1].parse::<usize>().unwrap();
            let from = cap[2].parse::<usize>().unwrap() - 1;
            let to = cap[3].parse::<usize>().unwrap() - 1;
            if part == 2 {
                let start = stack[from].len() - num_move;
                let mut to_move = stack[from].split_off(start);
                stack[to].append(&mut to_move);
            } else {
                for _ in 0..num_move {
                    match stack[from].pop() {
                        Some(to_move) => stack[to].push(to_move),
                        None => panic!("WWAA"),
                    };
                }
            }
        }
        println!("{:?}", stack);
    }
    println!("{:?}", stack);
    let mut answer = String::new();
    for mut column in stack {
        answer.push_str(column.pop().unwrap());
    }
    println!("The answer is {answer}");
}
