use ndarray::{s, Array};

pub fn run(input: &str, part: u8) -> u32 {
    let in_a_row = input.split_once('\n').unwrap().0.chars().count();
    let no_whitespace = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    // println!("There are {} in a row", in_a_row);
    let total = no_whitespace.len();
    let in_a_column = total / in_a_row;
    // println!("There are {} in total", total);

    let a = Array::from_shape_vec((in_a_row, in_a_column), no_whitespace).unwrap();
    let mut can_be_seen = Array::from_elem((in_a_row, in_a_column), false);
    // println!("{:?}", a);
    let mut cc = 0;
    for col in a.columns() {
        // Look west -> east
        for (rr, ele) in col.iter().enumerate() {
            // println!("{}", ele);
            // if ele > col.slice(s![..cc]) {
            if col.slice(s![..rr]).iter().all(|x| x < ele) {
                // println!("I can see {} {}", rr, cc);
                can_be_seen[[rr, cc]] = true;
            }
        }
        // Look east -> west
        for (rr, ele) in col.iter().enumerate().rev() {
            // println!("{} {}", rr, ele);
            // println!("{:?}", col.slice(s![rr + 1..]));
            // if ele > col.slice(s![..cc]) {
            if col.slice(s![rr + 1..]).iter().all(|x| x < ele) {
                // println!("I can see {} {}", rr, cc);
                can_be_seen[[rr, cc]] = true;
            }
        }
        cc += 1;
    }
    let mut rr = 0;
    for row in a.rows() {
        // Look north -> south
        for (cc, ele) in row.iter().enumerate() {
            // println!("{}", ele);
            // if ele > row.slice(s![..cc]) {
            if row.slice(s![..cc]).iter().all(|x| x < ele) {
                // println!("I can see {} {}", rr, cc);
                can_be_seen[[rr, cc]] = true;
            }
        }
        // Look south -> north
        for (cc, ele) in row.iter().enumerate().rev() {
            // println!("{} {}", cc, ele);
            // println!("{:?}", row.slice(s![cc + 1..]));
            if row.slice(s![cc + 1..]).iter().all(|x| x < ele) {
                // println!("I can see {} {}", rr, cc);
                can_be_seen[[rr, cc]] = true;
            }
        }
        rr += 1;
    }
    // println!("Can I see? \n {:?}", can_be_seen);
    return can_be_seen.iter().filter(|&x| *x == true).count() as u32;
}

#[test]
fn tetst_part1() {
    let input = include_str!("../inp/day8/test.txt");
    println!("{}", input);
    assert_eq!(run(input, 1), 21)
}
