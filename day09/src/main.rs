use std::fs;

fn decompress_len(s: &str) -> usize {
    let chars: Vec<char> = s.trim().chars().collect();

    let mut result = String::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '(' {
            result.push(chars[i]);
            i += 1;
            continue;
        }

        i += 1;
        let mut marker = String::new();
        while chars[i] != ')' {
            marker.push(chars[i]);
            i += 1;
        }

        i += 1;

        let parts = marker.splitn(2, "x").collect::<Vec<&str>>();
        let num = parts[0].parse::<usize>().unwrap();
        let rpt = parts[1].parse::<usize>().unwrap();

        let mut repeated = String::new();
        for _ in 0..num {
            repeated.push(chars[i]);
            i += 1;
        }

        for _ in 0..rpt {
            result.push_str(&repeated);
        }
    }

    result.len()
}

fn decompress_v2_len(s: &str) -> usize {
    let chars: Vec<char> = s.trim().chars().collect();

    let mut result = 0;
    let mut i = 0;

    while i < chars.len() {
        if chars[i] != '(' {
            result += 1;
            i += 1;
            continue;
        }

        i += 1;
        let mut marker = String::new();
        while chars[i] != ')' {
            marker.push(chars[i]);
            i += 1;
        }

        i += 1;
        let parts = marker.splitn(2, "x").collect::<Vec<&str>>();
        let num = parts[0].parse::<usize>().unwrap();
        let rpt = parts[1].parse::<usize>().unwrap();

        let mut repeated = String::new();
        for _ in 0..num {
            repeated.push(chars[i]);
            i += 1;
        }

        result += decompress_v2_len(&repeated) * rpt;
    }

    result
}

fn main() {
    let input = fs::read_to_string("inputs/day09.txt").unwrap();

    println!("v1: {}", decompress_len(&input));
    println!("v2: {}", decompress_v2_len(&input))
}
