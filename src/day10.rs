const CHECKS: [u32; 6] = [20, 60, 100, 140, 180, 220];
const SCREEN_SIZE: usize = 240;
pub const LINE_WIDTH: usize = 40;

pub fn run(input: &str, part: u8) -> (i32, [char; SCREEN_SIZE]) {
    let mut cycle = 0;
    let mut image: [char; SCREEN_SIZE] = ['.'; SCREEN_SIZE];
    let mut x: i32 = 1;
    let mut ans = 0;
    for line in input.split('\n') {
        if line == "noop" {
            if part == 1 {
                increment_cycle(&mut cycle);
            } else {
                increment_cycle_part2(&mut cycle, &mut image, &x);
            }
            if CHECKS.contains(&cycle) {
                ans += x * cycle as i32;
            }
        } else if line.is_empty() {
        } else {
            let (_, value) = line.split_once(' ').unwrap();
            for _ in 1..=2 {
                if part == 1 {
                    increment_cycle(&mut cycle);
                } else {
                    increment_cycle_part2(&mut cycle, &mut image, &x);
                }
                if CHECKS.contains(&cycle) {
                    ans += x * cycle as i32;
                }
            }
            let to_add = value.parse::<i32>().unwrap();
            x += to_add;
        }
    }

    (ans, image)
}

fn increment_cycle(cycle: &mut u32) {
    *cycle += 1;
}

fn increment_cycle_part2(cycle: &mut u32, image: &mut [char; SCREEN_SIZE], head_position: &i32) {
    let new_cycle: i32 = (*cycle as usize % LINE_WIDTH) as i32;

    if (head_position - 1..=head_position + 1).contains(&new_cycle) {
        image[*cycle as usize] = '#';
    }

    increment_cycle(cycle);
}

#[test]
fn test_noddy() {
    let input = "noop
addx 3
addx -5";
    assert_eq!(run(input, 1).0, 0);
}

#[test]
fn test_division_understanding() {
    assert_eq!(40 / 4, 10);
    assert_eq!(40 / 3, 13);
    assert_eq!(40 % 3, 1);
}

#[test]
fn test_part1() {
    let input = include_str!("../inp/day10/test.txt");
    assert_eq!(run(input, 1).0, 13140);
}

#[test]
fn test_part2() {
    let input = include_str!("../inp/day10/test.txt");
    let ans = "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....";
    assert_eq!(run(input, 2).1.iter().collect::<String>(), ans);
}
