extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::fs;

fn has_abba(s: &str) -> bool {
    let data: Vec<char> = s.chars().collect();

    for (i, c) in data.iter().enumerate() {
        if i + 3 >= data.len() {
            break;
        }

        if data[i + 1] != *c && data[i + 2] == data[i + 1] && data[i + 3] == *c {
            return true;
        }
    }

    false
}

fn get_bab_matches(s: &str) -> Vec<String> {
    let mut matches: HashSet<String> = HashSet::new();
    let data: Vec<char> = s.chars().collect();

    for (i, c) in data.iter().enumerate() {
        if i + 2 >= data.len() {
            break;
        }

        let c2 = data[i + 1];
        if c2 != ' ' && c2 != *c && data[i + 2] == *c {
            matches.insert(vec![c2, *c, c2].into_iter().collect());
        }
    }

    matches.into_iter().collect()
}

fn main() {
    let re = Regex::new(r"\[(?P<in>[a-z]+)\]").unwrap();
    let input = fs::read_to_string("inputs/day07.txt").unwrap();

    let mut supports_tls = 0;
    let mut supports_ssl = 0;

    for l in input.lines() {
        let replaced = re.replace_all(l, " ");
        let hypernet_strs: Vec<&str> = re
            .captures_iter(l)
            .map(|c| c.name("in").unwrap().as_str())
            .collect();

        if has_abba(&replaced) && !hypernet_strs.iter().any(|s| has_abba(s)) {
            supports_tls += 1;
        }

        let has_bab = get_bab_matches(&replaced)
            .iter()
            .any(|m| hypernet_strs.iter().any(|s| s.contains(m)));

        if has_bab {
            supports_ssl += 1;
        }
    }

    println!("supported tls ip addrs: {}", supports_tls);
    println!("supported ssl ip addrs: {}", supports_ssl);
}
