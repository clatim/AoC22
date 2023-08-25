use std::collections::HashMap;

static DISC_SIZE: u64 = 70_000_000;
static REQUIRED_SPACE: u64 = 30_000_000;

pub fn run(input: &str, part: u8) -> u64 {
    let mut sizes: HashMap<String, u64> = HashMap::new();
    // println!("{}", input);
    let mut commands = input.split('$');
    // Skip empty line
    commands.next();
    let mut cwd = vec![];

    for line in commands {
        let (command, result) = line.split_once('\n').unwrap();
        // println!("command: {}", command);
        // println!("result: {}", result);

        let command_words = command.trim().split(' ').collect::<Vec<&str>>();
        let part1 = *command_words
            .get(0)
            .expect("There should be at least 1 entry on a command!");

        if part1 == "cd" {
            let part2 = command_words.get(1).unwrap();
            if *part2 == ".." {
                cwd.pop();
            } else {
                cwd.push(String::from(*part2));
            }
            // println!("The current working directory is {}", cwd.join("/"));
        } else if part1 == "ls" {
            for word in result.split('\n') {
                if let Some((left, _)) = word.split_once(' ') {
                    if left != "dir" {
                        let size: u64 = left.parse().expect("First entry should be an integer.");

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
        return sizes.values().filter(|v| v <= &&100_000u64).sum();
    } else if part == 2 {
        let free_memory = DISC_SIZE - sizes.get("/").unwrap();
        return sizes
            .values()
            .filter(|v| free_memory + **v > REQUIRED_SPACE)
            .min()
            .expect("The iterator should not be empty.")
            .clone();
    }
    return 1;
}

#[test]
fn test_part1() {
    let input = include_str!("../inp/day7/test.txt");
    assert_eq!(run(input, 1), 95437)
}

#[test]
fn test_part2() {
    let input = include_str!("../inp/day7/test.txt");
    assert_eq!(run(input, 2), 24933642)
}
