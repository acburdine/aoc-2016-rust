use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Cardinal {
    fn turn(&self, dir: Direction) -> Cardinal {
        match self {
            Cardinal::North => match dir {
                Direction::Left => Cardinal::West,
                Direction::Right => Cardinal::East,
            },
            Cardinal::South => match dir {
                Direction::Left => Cardinal::East,
                Direction::Right => Cardinal::West,
            },
            Cardinal::East => match dir {
                Direction::Left => Cardinal::North,
                Direction::Right => Cardinal::South,
            },
            Cardinal::West => match dir {
                Direction::Left => Cardinal::South,
                Direction::Right => Cardinal::North,
            },
        }
    }
}

#[derive(Copy, Clone)]
struct Instruction {
    dir: Direction,
    blocks: isize,
}

impl Instruction {
    fn new(s: &str) -> Instruction {
        let parts: Vec<&str> = s.splitn(3, "").filter(|i| *i != "").collect();
        Instruction {
            dir: match parts[0] {
                "R" => Direction::Right,
                "L" => Direction::Left,
                _ => panic!("unrecognized direction: {}", parts[0]),
            },
            blocks: parts[1].parse().unwrap(),
        }
    }

    fn follow(
        &self,
        seen: &mut HashSet<(isize, isize)>,
        card: Cardinal,
        mut x: isize,
        mut y: isize,
    ) -> (Cardinal, isize, isize, Option<isize>) {
        let new_card = card.turn(self.dir);
        let mut actual_place = None;

        match new_card {
            Cardinal::North => {
                for _ in 0..self.blocks {
                    y += 1;
                    if !seen.insert((x, y)) && actual_place == None {
                        actual_place = Some(x.abs() + y.abs());
                    }
                }
            }
            Cardinal::South => {
                for _ in 0..self.blocks {
                    y -= 1;
                    if !seen.insert((x, y)) && actual_place == None {
                        actual_place = Some(x.abs() + y.abs());
                    }
                }
            }
            Cardinal::East => {
                for _ in 0..self.blocks {
                    x += 1;
                    if !seen.insert((x, y)) && actual_place == None {
                        actual_place = Some(x.abs() + y.abs());
                    }
                }
            }
            Cardinal::West => {
                for _ in 0..self.blocks {
                    x -= 1;
                    if !seen.insert((x, y)) && actual_place == None {
                        actual_place = Some(x.abs() + y.abs());
                    }
                }
            }
        };

        return (new_card, x, y, actual_place);
    }
}

fn main() {
    let input: Vec<Instruction> = fs::read_to_string("inputs/day01.txt")
        .unwrap()
        .trim()
        .split(", ")
        .map(|s| Instruction::new(s))
        .collect();

    let mut pos = (Cardinal::North, 0, 0, None);
    let mut set: HashSet<(isize, isize)> = HashSet::new();
    let mut found_actual = false;

    input.into_iter().for_each(|i| {
        pos = i.follow(&mut set, pos.0, pos.1, pos.2);

        if !found_actual {
            if let Some(v) = pos.3 {
                println!("actual distance: {}", v);
                found_actual = true;
            }
        }
    });

    let x = pos.1.abs();
    let y = pos.2.abs();

    println!("distance: {}", x + y);
}
