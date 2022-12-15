pub fn run_day6(input: &str, part: u8) -> usize {
    let letters: Vec<String> = input.chars().map(|c| c.to_string()).collect();
    let num_chars = match part {
        1 => 4,
        2 => 14,
        _ => panic!("There are only two parts to this!"),
    };
    let mut ct = num_chars;
    let mut total_matches;

    for compare in letters.windows(num_chars) {
        total_matches = 0;
        println!("{:?}", compare);

        for entry in compare.iter() {
            total_matches += compare.iter().filter(|n| *n == entry).count();
        }

        if total_matches == num_chars {
            println!("The answer is {}", ct);
            return ct;
        }
        ct += 1;

    }
    0
}

#[test]
fn first_example_part_1() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(run_day6(input, 1), 7);
}

#[test]
fn second_example_part_1() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(run_day6(input, 1), 5);
}

#[test]
fn third_example_part_1() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(run_day6(input, 1), 6);
}

#[test]
fn fourth_example_part_1() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(run_day6(input, 1), 10);
}

#[test]
fn fifth_example_part_1() {
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(run_day6(input, 1), 11);
}

#[test]
fn first_example_part_2() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(run_day6(input, 2), 19);
}

#[test]
fn second_example_part_2() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(run_day6(input, 2), 23);
}

#[test]
fn third_example_part_2() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(run_day6(input, 2), 23);
}

#[test]
fn fourth_example_part_2() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(run_day6(input, 2), 29);
}

#[test]
fn fifth_example_part_2() {
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(run_day6(input, 2), 26);
}

