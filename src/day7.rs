use std::collections::HashMap;

static DISC_SIZE: u64 = 70_000_000;
static REQUIRED_SPACE: u64 = 30_000_000;

pub fn run(input: &str, part: u8) -> u64 {
    let mut sizes: HashMap<String, u64> = HashMap::new();
    let mut commands = input.split('$');
    // Skip empty line
    commands.next();
    let mut cwd = vec![];

    for line in commands {
        let (command, result) = line.split_once('\n').unwrap();
        let command_words = command.split_whitespace().collect::<Vec<&str>>();

        match command_words[..] {
            ["cd", ".."] => {
                cwd.pop();
            }
            ["cd", name] => {
                cwd.push(name);
            }
            ["ls"] => {
                let sizers = result
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .filter_map(|e| e.parse::<u64>().ok());
                for size in sizers {
                    for idx in 0..cwd.len() {
                        let path = cwd[..=idx].join("/");
                        *sizes.entry(path).or_insert(0) += size;
                    }
                }
            }
            _ => {}
        };
    }

    if part == 1 {
        return sizes.into_values().filter(|v| v <= &100_000u64).sum();
    } else if part == 2 {
        let free_memory = DISC_SIZE - sizes.get("/").unwrap();
        return sizes
            .into_values()
            .filter(|v| free_memory + v > REQUIRED_SPACE)
            .min()
            .expect("The iterator should not be empty.");
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
