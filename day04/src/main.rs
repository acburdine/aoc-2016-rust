extern crate regex;

use regex::{Captures, Regex};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

const ROOM_REG: &str = r"^(?P<name>[-a-z]+)(?P<sectorid>[0-9]+)\[(?P<checksum>[a-z]+)\]$";
const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

struct Room {
    name: Vec<char>,
    sector_id: usize,
    checksum: String,
}

impl Room {
    fn new(matches: Captures) -> Room {
        Room {
            name: matches.name("name").unwrap().as_str().chars().collect(),
            sector_id: matches.name("sectorid").unwrap().as_str().parse().unwrap(),
            checksum: matches.name("checksum").unwrap().as_str().chars().collect(),
        }
    }

    fn valid(&self) -> bool {
        let mut char_map: HashMap<char, usize> = HashMap::new();

        for c in self.name.iter() {
            if *c == '-' {
                continue;
            }

            char_map.entry(*c).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut letters: Vec<char> = char_map.keys().map(|c| *c).collect();
        letters.sort_by(
            |a, b| match char_map.get(b).unwrap().cmp(char_map.get(a).unwrap()) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => a.cmp(b),
            },
        );

        let checksum: String = letters.into_iter().take(5).collect();

        checksum.eq(&self.checksum)
    }

    fn decrypt(&self) -> String {
        let mut decrypted: Vec<char> = Vec::with_capacity(self.name.len());

        for c in self.name.iter() {
            if *c == '-' {
                decrypted.push(' ');
                continue;
            }

            let current_index = ALPHABET.iter().position(|p| p == c).unwrap();
            decrypted.push(ALPHABET[(current_index + self.sector_id) % ALPHABET.len()]);
        }

        decrypted.into_iter().collect()
    }
}

fn main() {
    let re = Regex::new(ROOM_REG).unwrap();
    let input = fs::read_to_string("inputs/day04.txt").unwrap();

    let rooms: Vec<Room> = input
        .lines()
        .map(|l| Room::new(re.captures(l).unwrap()))
        .collect();

    println!(
        "northpole room: {}",
        rooms
            .iter()
            .find(|r| r.decrypt().trim().eq("northpole object storage"))
            .unwrap()
            .sector_id
    );

    let sum: usize = rooms
        .iter()
        .filter(|r| r.valid())
        .map(|r| r.sector_id)
        .sum();

    println!("sum of valid rooms: {}", sum);
}
