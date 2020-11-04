extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

const INPUT: &str = "ffykfhsq";

fn hash(number: usize) -> Option<(char, char)> {
    let mut hasher = Md5::new();
    hasher.input_str(INPUT);
    hasher.input_str(number.to_string().as_str());

    let result = hasher.result_str();
    if result.chars().take(5).all(|c| c == '0') {
        return Some((
            result.chars().nth(5).unwrap(),
            result.chars().nth(6).unwrap(),
        ));
    }

    None
}

fn main() {
    let mut password = String::new();
    let mut password_v2 = [' '; 8];

    for i in 0..std::usize::MAX {
        if password.len() == 8 && password_v2.iter().all(|c| *c != ' ') {
            break;
        }

        match hash(i) {
            Some((i, c)) => {
                if password.len() < 8 {
                    password.push(i);
                }

                match i.to_digit(10) {
                    None => (),
                    Some(d) => {
                        if d <= 7 && password_v2[d as usize] == ' ' {
                            password_v2[d as usize] = c;
                        }
                    }
                }
            }
            None => (),
        }
    }

    println!("password: {}", password);
    println!("password v2: {}", password_v2.iter().collect::<String>());
}
