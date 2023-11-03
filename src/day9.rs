use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Direction {
    U,
    D,
    L,
    R,
    UR,
    UL,
    DL,
    DR,
}

const MOVES: [Direction; 8] = [Direction::R, Direction::L, Direction::U, Direction::D, Direction::UR, Direction::DR, Direction::UL, Direction::DL];
impl Direction {
    fn new(direction: &str) -> Self {
        match direction {
            "R" => Direction::R,
            "L" => Direction::L,
            "U" => Direction::U,
            "D" => Direction::D,
            "UR" => Direction::UR,
            "UL" => Direction::UL,
            "DL" => Direction::DL,
            "DR" => Direction::DR,
            _ => panic!("Haven't done the direction {}", direction),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Knot {
    x: isize,
    y: isize,
}

impl Knot {
    fn move_the_thing(&mut self, direction: &Direction) {
        // Given a Direction, this moves the knot in a given direction by 1 in that direction.
        // If it is a diagonal move then it will be a move by 1 in each axis - or a displacement
        // on sqrt(2).
        match direction {
            Direction::R => self.x += 1,
            Direction::L => self.x -= 1,
            Direction::U => self.y += 1,
            Direction::D => self.y -= 1,
            Direction::UR => {
                self.x += 1;
                self.y += 1;
            }
            Direction::UL => {
                self.x -= 1;
                self.y += 1;
            }
            Direction::DL => {
                self.x -= 1;
                self.y -= 1;
            }
            Direction::DR => {
                self.x += 1;
                self.y -= 1;
            }
        };
        // println!("moved to {:?}", self);
    }

    fn find_the_move(&self, leader: &Knot) -> Direction {
        // scans through all possible diagonal moves and finds the move that minimises the distance
        // to the leader.
        let places = [
            Knot {
                x: self.x + 1,
                y: self.y,
            },
            Knot {
                x: self.x - 1,
                y: self.y,
            },
            Knot {
                x: self.x,
                y: self.y + 1,
            },
            Knot {
                x: self.x,
                y: self.y - 1,
            },
            Knot {
                x: self.x + 1,
                y: self.y + 1,
            },
            Knot {
                x: self.x + 1,
                y: self.y - 1,
            },
            Knot {
                x: self.x - 1,
                y: self.y + 1,
            },
            Knot {
                x: self.x - 1,
                y: self.y - 1,
            },
        ]
        .map(|k| leader.distance_between(&k));

        let min_index = places
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();
        // println!("This is min {:?}", min_index);
        // println!("Move me {:?}", MOVES[min_index.0]);

        MOVES[min_index.0].clone()
    }
    fn distance_between(&self, other_knot: &Knot) -> f32 {
        // Computes the distance between two Knots.
        distance_between_two_points((&self.x, &self.y), (&other_knot.x, &other_knot.y))
    }
}

fn distance_between_two_points(one: (&isize, &isize), two: (&isize, &isize)) -> f32 {
    (((one.0 - two.0).pow(2) + (one.1 - two.1).pow(2)) as f32).sqrt()
}

pub fn run(input: &str, part: u8) -> u32 {
    // Okay, lets start by thinking how to move the head from the input.
    let mut parts = vec![];
    const LONG_ROPE: usize = 10;
    let length_of_rope = match part {
        1 => 2,
        2 => LONG_ROPE,
        _ => panic!("part should be in 1,2"),
    };
    for _ in 0..length_of_rope {
        parts.push(Knot { x: 0, y: 0 });
    }
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    for command in input.split('\n') {
        if command.is_empty() {
            // Skip the last empty line after final \n
            continue;
            // use a hashset ot store the locations
        }
        // println!("{}", command);

        let (direction_str, magnitude) = command.split_once(' ').expect("Split_once should work");
        let direction = Direction::new(direction_str);
        let magnitude = magnitude.parse::<i8>().unwrap();
        for _ in 0..magnitude {
            // println!("moving head");
            parts[0].move_the_thing(&direction);
            for part in 0..length_of_rope - 1 {
                let follow = part + 1;
                let distance = parts[part].distance_between(&parts[follow]);
                // println!("i am part {} with distance {}", follow, distance);
                if distance >= 2.0 {
                    let direction = parts[follow].find_the_move(&parts[part]);
                    parts[follow].move_the_thing(&direction);
                }
                // if distance == 2.0 {
                //     parts[follow].move_the_thing(&direction);
                // } else if distance > 2.0 {
                //     let direction = parts[follow].find_the_move(&parts[part]);
                //     parts[follow].move_the_thing(&direction);
                // }
            }
            visited.insert((parts[length_of_rope - 1].x, parts[length_of_rope - 1].y));
        }
        // for (id, part) in parts.iter().enumerate() {
        //     println!("part {} is at {:?}", id, part);
        // }
    }
    visited.len() as u32
}

#[test]
fn test_part1() {
    let input = include_str!("../inp/day9/test.txt");
    assert_eq!(run(input, 1), 13);
}

#[test]
fn test_part2() {
    let input = include_str!("../inp/day9/test.txt");
    assert_eq!(run(input, 2), 1);
}

#[test]
fn test_part2_bigger() {
    let input = include_str!("../inp/day9/test_larger.txt");
    assert_eq!(run(input, 2), 36);
}
