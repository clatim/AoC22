use std::collections::HashMap;

pub fn run(input: &str, part: u8) -> u32 {
    if part != 1 {
        panic!("Day 7 has only had the first part implemented.");
    }

    let mut sizes: HashMap<String, u32> = HashMap::new();
    // println!("{}", input);
    let mut commands = input.split('$');
    // Skip empty line
    commands.next();
    let mut cwd = vec![];

    for line in commands {
        let (command, result) = line.split_once('\n').unwrap();
        let command = String::from(command);
        // println!("command: {}", command);
        // println!("result: {}", result);

        let command_words = command.trim().split(' ').collect::<Vec<&str>>();
        // println!("command_words = {:?}", command_words);
        let part1 = command_words.get(0).unwrap().clone();
        // println!("First part {}", part1);
        if part1 == "cd" {
            let part2 = String::from(command_words.get(1).unwrap().clone());
            if part2.as_str() == ".." {
                cwd.pop();
            } else {
                cwd.push(part2);
            }
            println!("The current working directory is {}", cwd.join("/"));
        } else if part1 == "ls" {
            for word in result.split('\n') {
                if let Some((left, _)) = word.split_once(' ') {
                    if left != "dir" {
                        let size: u32 = left.parse().expect("First entry should be an integer.");

                        for idx in 0..cwd.len() {
                            let path = cwd[..=idx].join("/");
                            *sizes.entry(path).or_insert(0) += size;
                        }
                    }
                }
            }
        }
    }

    if part == 1 {
        return sizes.values().filter(|v| v <= &&100_000u32).sum();
    }
    return 1;
}

#[test]
fn test_part1() {
    let input = include_str!("../inp/day7/test.txt");
    assert_eq!(run(input, 1), 95437)
}
