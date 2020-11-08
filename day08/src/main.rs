extern crate regex;

use regex::Regex;
use std::fs;

#[derive(Clone, Copy)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl Instruction {
    fn run(self, old_screen: [[bool; 50]; 6]) -> [[bool; 50]; 6] {
        let mut screen = old_screen.clone();

        match self {
            Instruction::Rect(x_limit, y_limit) => {
                for y in 0..y_limit {
                    for x in 0..x_limit {
                        screen[y][x] = true;
                    }
                }
                screen
            }
            Instruction::RotateRow(y, amt) => {
                for x in 0..50 {
                    let next_x = (x + amt) % 50;
                    screen[y][next_x] = old_screen[y][x];
                }
                screen
            }
            Instruction::RotateColumn(x, amt) => {
                for y in 0..6 {
                    let next_y = (y + amt) % 6;
                    screen[next_y][x] = old_screen[y][x];
                }
                screen
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day08.txt").unwrap();
    let rotate_regex =
        Regex::new(r"(?P<dir>row|column) (?:x|y)=(?P<num>[0-9]+) by (?P<amt>[0-9]+)").unwrap();

    let mut instrs: Vec<Instruction> = Vec::new();
    for l in input.trim().lines() {
        let split_a: Vec<&str> = l.splitn(2, " ").collect();

        match split_a[0] {
            "rect" => {
                let split_rect: Vec<usize> = split_a[1]
                    .splitn(2, "x")
                    .map(|i| i.parse().unwrap())
                    .collect();
                instrs.push(Instruction::Rect(split_rect[0], split_rect[1]));
            }
            "rotate" => {
                let matches = rotate_regex.captures(split_a[1]).unwrap();
                let rotate_dir = matches.name("dir").unwrap().as_str();
                let num: usize = matches.name("num").unwrap().as_str().parse().unwrap();
                let amt: usize = matches.name("amt").unwrap().as_str().parse().unwrap();

                match rotate_dir {
                    "row" => {
                        instrs.push(Instruction::RotateRow(num, amt));
                    }
                    "column" => {
                        instrs.push(Instruction::RotateColumn(num, amt));
                    }
                    s => panic!("unrecognized direction: {}", s),
                }
            }
            s => panic!("unknown instruction {}", s),
        }
    }

    let mut screen = [[false; 50]; 6];
    for instr in instrs.iter() {
        screen = instr.run(screen);
    }

    let sum: usize = screen
        .iter()
        .map(|row| row.iter().filter(|v| **v).count())
        .sum();

    println!("lit: {}", sum);
    for y in 0..6 {
        for x in 0..50 {
            if screen[y][x] {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!("");
    }
}
