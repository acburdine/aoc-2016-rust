extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone)]
struct Bot {
    low: Give,
    high: Give,
    run: bool,
    chip_1: Option<usize>,
    chip_2: Option<usize>,
}

impl Bot {
    fn new(low: Give, high: Give) -> Bot {
        Bot {
            low,
            high,
            run: false,
            chip_1: None,
            chip_2: None,
        }
    }

    fn can_run(&self) -> bool {
        !self.run && self.chip_1 != None && self.chip_2 != None
    }

    fn run(&mut self, outputs: &mut HashMap<usize, usize>, bots: &mut HashMap<usize, Bot>) {
        if self.chip_1 == None || self.chip_2 == None {
            panic!("bot doesn't have two chips!");
        }

        let chips = vec![self.chip_1.unwrap(), self.chip_2.unwrap()];

        let high = *chips.iter().max().unwrap();
        match self.high {
            Give::Output(i) => {
                outputs.insert(i, high);
            }
            Give::Bot(i) => {
                bots.get_mut(&i).unwrap().give_chip(high);
            }
        }

        let low = *chips.iter().min().unwrap();
        match self.low {
            Give::Output(i) => {
                outputs.insert(i, low);
            }
            Give::Bot(i) => {
                bots.get_mut(&i).unwrap().give_chip(low);
            }
        }

        self.run = true;
    }

    fn give_chip(&mut self, chip: usize) {
        if self.chip_1 == None {
            self.chip_1 = Some(chip);
            return;
        }

        if self.chip_2 == None {
            self.chip_2 = Some(chip);
            return;
        }

        panic!("bot already has two chips!");
    }

    fn match_case(&self) -> bool {
        self.chip_1 == Some(61) && self.chip_2 == Some(17)
    }
}

#[derive(Copy, Clone)]
enum Give {
    Output(usize),
    Bot(usize),
}

impl Give {
    fn parse(s: &str) -> Give {
        let parts: Vec<&str> = s.trim().splitn(2, " ").collect();
        match parts[0] {
            "output" => Give::Output(parts[1].parse().unwrap()),
            "bot" => Give::Bot(parts[1].parse().unwrap()),
            p => panic!("unable to parse {} as Give type", p),
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day10.txt").unwrap();
    let value_re = Regex::new(r"^value (?P<val>[0-9]+) goes to bot (?P<bot>[0-9]+)$").unwrap();
    let bot_re = Regex::new(r"^bot (?P<src>[0-9]+) gives low to (?P<low>(?:output|bot) [0-9]+) and high to (?P<high>(?:output|bot) [0-9]+)$").unwrap();

    let mut input_chips: Vec<&str> = Vec::new();
    let mut bots: HashMap<usize, Bot> = HashMap::new();
    let mut outputs: HashMap<usize, usize> = HashMap::new();

    for l in input.trim().lines() {
        if l.starts_with("value") {
            input_chips.push(l);
            continue;
        }

        let matches = bot_re.captures(l).unwrap();
        let bot_id = matches.name("src").unwrap().as_str().parse().unwrap();
        let low = Give::parse(matches.name("low").unwrap().as_str());
        let high = Give::parse(matches.name("high").unwrap().as_str());

        bots.insert(bot_id, Bot::new(low, high));
    }

    for v in input_chips {
        let matches = value_re.captures(v).unwrap();
        let bot_id = matches.name("bot").unwrap().as_str().parse().unwrap();
        let val = matches.name("val").unwrap().as_str().parse().unwrap();

        bots.get_mut(&bot_id).unwrap().give_chip(val);
    }

    while bots.iter().filter(|(_, b)| !b.run).count() > 0 {
        for (i, mut b) in bots.clone().into_iter().filter(|(_, b)| b.can_run()) {
            if b.match_case() {
                println!("bot comparing 61 with 17: {}", i);
            }

            b.run(&mut outputs, &mut bots);
            bots.insert(i, b);
        }
    }

    println!(
        "multiplication: {}",
        outputs[&0] * outputs[&1] * outputs[&2]
    );
}
