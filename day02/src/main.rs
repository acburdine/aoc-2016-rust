use std::fs;

fn code_v1(input: &str) -> String {
    let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    let mut x = 1;
    let mut y = 1;

    let mut code: Vec<char> = Vec::new();

    for l in input.trim().lines() {
        for c in l.chars() {
            match c {
                'L' => {
                    if x > 0 {
                        x -= 1;
                    }
                }
                'R' => {
                    if x < 2 {
                        x += 1;
                    }
                }
                'U' => {
                    if y > 0 {
                        y -= 1;
                    }
                }
                'D' => {
                    if y < 2 {
                        y += 1;
                    }
                }
                _ => panic!("unrecognized instruction: {}", c),
            }
        }

        code.push(keypad[y][x]);
    }

    code.into_iter().collect()
}

fn code_v2(input: &str) -> String {
    let keypad = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];

    let mut x = 0;
    let mut y = 2;

    let mut code: Vec<char> = Vec::new();

    for l in input.trim().lines() {
        for c in l.chars() {
            match c {
                'L' => {
                    if x > 0 {
                        x -= 1;

                        if keypad[y][x] == ' ' {
                            x += 1;
                        }
                    }
                }
                'R' => {
                    if x < 4 {
                        x += 1;

                        if keypad[y][x] == ' ' {
                            x -= 1;
                        }
                    }
                }
                'U' => {
                    if y > 0 {
                        y -= 1;

                        if keypad[y][x] == ' ' {
                            y += 1;
                        }
                    }
                }
                'D' => {
                    if y < 4 {
                        y += 1;

                        if keypad[y][x] == ' ' {
                            y -= 1;
                        }
                    }
                }
                _ => panic!("unrecognized instruction: {}", c),
            }
        }

        code.push(keypad[y][x]);
    }

    code.into_iter().collect()
}

fn main() {
    let input = fs::read_to_string("inputs/day02.txt").unwrap();

    println!("part 1: {}", code_v1(&input));
    println!("part 2: {}", code_v2(&input));
}
