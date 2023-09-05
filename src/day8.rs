use ndarray::{s, Array, Array2, Axis};

pub fn run(input: &str, part: u8) -> u32 {
    let in_a_row = input.split_once('\n').unwrap().0.chars().count();
    let no_whitespace = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let total = no_whitespace.len();
    let in_a_column = total / in_a_row;

    let a = Array::from_shape_vec((in_a_row, in_a_column), no_whitespace).unwrap();
    if part == 1 {
        return find_trees_that_can_be_seen(&a)
            .iter()
            .filter(|&x| *x == true)
            .count() as u32;
    } else {
        return *find_trees_viewing_distance(&a)
            .iter()
            .max()
            .expect("Viewing distance should not be empty.") as u32;
    }
}

fn find_trees_that_can_be_seen<T: std::cmp::PartialOrd>(
    canopy_heights: &Array2<T>,
) -> Array2<bool> {
    // Does part 1, that is count the number of trees that can be seen from the outside
    // of the forest.
    let mut can_be_seen = Array::from_elem(
        (
            canopy_heights.len_of(Axis(0)),
            canopy_heights.len_of(Axis(1)),
        ),
        false,
    );

    for (cc, col) in canopy_heights.columns().into_iter().enumerate() {
        // Look west -> east
        for (rr, ele) in col.iter().enumerate() {
            if col.slice(s![..rr]).iter().all(|x| x < ele) {
                can_be_seen[[rr, cc]] = true;
            }
        }
        // Look east -> west
        for (rr, ele) in col.iter().enumerate().rev() {
            if col.slice(s![rr + 1..]).iter().all(|x| x < ele) {
                can_be_seen[[rr, cc]] = true;
            }
        }
    }
    for (rr, row) in canopy_heights.rows().into_iter().enumerate() {
        // Look north -> south
        for (cc, ele) in row.iter().enumerate() {
            if row.slice(s![..cc]).iter().all(|x| x < ele) {
                can_be_seen[[rr, cc]] = true;
            }
        }
        // Look south -> north
        for (cc, ele) in row.iter().enumerate().rev() {
            if row.slice(s![cc + 1..]).iter().all(|x| x < ele) {
                can_be_seen[[rr, cc]] = true;
            }
        }
    }
    return can_be_seen;
}

fn find_trees_viewing_distance<T>(canopy_heights: &Array2<T>) -> Array2<usize>
where
    T: std::cmp::PartialOrd,
{
    let mut viewing_distance = Array::from_elem(
        (
            canopy_heights.len_of(Axis(0)),
            canopy_heights.len_of(Axis(1)),
        ),
        1,
    );

    for (cc, col) in canopy_heights.columns().into_iter().enumerate() {
        // Look north
        for (rr, tree) in col.iter().enumerate() {
            // let current_distance = col.slice(s![..rr]).iter().skip_while(|x| *x < tree).count();
            let mut current_distance = 0;
            for see_tree in col.slice(s![..rr]).iter().rev() {
                current_distance += 1;
                if see_tree >= tree {
                    break;
                }
            }
            viewing_distance[[rr, cc]] *= current_distance;
        }
        // Look south
        for (rr, tree) in col.iter().enumerate().rev() {
            let mut current_distance = 0;
            for see_tree in col.slice(s![rr + 1..]).iter() {
                current_distance += 1;
                if see_tree >= tree {
                    break;
                }
            }
            viewing_distance[[rr, cc]] *= current_distance;
        }
    }
    for (rr, row) in canopy_heights.rows().into_iter().enumerate() {
        // Look west
        for (cc, tree) in row.iter().enumerate() {
            let mut current_distance = 0;
            for see_tree in row.slice(s![..cc]).iter().rev() {
                current_distance += 1;
                if see_tree >= tree {
                    break;
                }
            }
            viewing_distance[[rr, cc]] *= current_distance;
        }
        // Look east
        for (cc, tree) in row.iter().enumerate() {
            let mut current_distance = 0;
            for see_tree in row.slice(s![cc + 1..]) {
                current_distance += 1;
                if see_tree >= tree {
                    break;
                }
            }
            viewing_distance[[rr, cc]] *= current_distance;
        }
    }
    return viewing_distance;
}

#[test]
fn tetst_part1() {
    let input = include_str!("../inp/day8/test.txt");
    println!("{}", input);
    assert_eq!(run(input, 1), 21)
}

#[test]
fn test_part2() {
    let input = include_str!("../inp/day8/test.txt");
    assert_eq!(run(input, 2), 8)
}
