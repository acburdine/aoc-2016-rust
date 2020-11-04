use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day06.txt").unwrap();
    let mut counts: Vec<HashMap<char, usize>> = Vec::new();

    for l in input.lines() {
        if counts.len() == 0 {
            l.chars().for_each(|_| counts.push(HashMap::new()));
        }

        for (i, c) in l.char_indices() {
            counts[i].entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    let mut message = String::new();
    let mut actual_message = String::new();

    for map in counts {
        let mut keys: Vec<char> = map.keys().map(|c| *c).collect();
        keys.sort_by(|a, b| map[b].cmp(&map[a]));

        message.push(keys[0]);
        actual_message.push(*keys.last().unwrap());
    }

    println!("message: {}", message);
    println!("actual message: {}", actual_message);
}
