use std::collections::HashMap;

pub fn run(input: &str, part: u8) -> u32 {
    if part != 1 {
        panic!("Day 7 has only had the first part implemented.");
    }

    let mut sizes: HashMap<String, u32> = HashMap::new();
    // println!("{}", input);
    let mut commands = input.split('$').enumerate();
    commands.next();
    commands.next();
    // Will always want a root
    let mut cwd = vec![String::from("root")];
    sizes.insert(String::from("root"), 0);

    for (ii, line) in commands {
        // println!("{}: {:?}", ii, line);
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
            let words = result.split('\n');
            // println!("{:?}", words.clone().collect::<Vec<&str>>());
            for word in words {
                if let Some((left, right)) = word.split_once(' ') {
                    // println!("left: {}, right: {}", left, right);
                    if left == "dir" {
                        // println!("Add a dir called {}", right);
                    } else {
                        let size: u32 = left.parse().unwrap();

                        for dir in cwd.iter() {
                            let current_size = sizes.entry(dir.clone()).or_insert(0);
                            *current_size += size;
                        }
                        println!(
                            "Add a file called {} with size {} to {:?}",
                            right, size, cwd,
                        );
                    }
                }
            }
        }
    }

    println!("\n");
    println!("The directory sizes are:");
    for (key, val) in sizes.iter() {
        println!("dir {key} size {val}");
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
