const CHECKS: [u32; 6] = [20, 60, 100, 140, 180, 220];

pub fn run(input: &str, part: u8) -> i32 {
    let mut cycle = 0;
    let mut value = 0;
    let mut x: i32 = 1;
    let mut ans = 0;
    for line in input.split('\n') {
        // println!("line is{}", line);
        let mut num_cycles = 0;
        let command: &str;
        let mut value: &str = "0";
        if line == "noop" {
            cycle += 1;
            if CHECKS.contains(&cycle) {
                println!("Adding {}", x*cycle as i32);
                ans += x * cycle as i32;
            }
        } else if line == "" {
            // println!("super empty");
        } else {
            (command, value) = line.split_once(' ').unwrap();
            for _ in 1..=2{
                cycle += 1;
                if CHECKS.contains(&cycle) {
                    println!("Adding {}", x*cycle as i32);
                    ans += x * cycle as i32;
                }
            }
            let to_add = value.parse::<i32>().unwrap();
            // println!("Adding {to_add}");
            x += to_add;
        }
        println!("It is cycle {cycle} with value {x}");

    }
    ans
    
}

#[test]
fn test_noddy() {
    let input = "noop
addx 3
addx -5";
    assert_eq!(run(input, 1), 0);
}
    
#[test]
fn test_part1() {
    let input = include_str!("../inp/day10/test.txt");
    assert_eq!(run(input, 1), 13140);
}
