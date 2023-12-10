use crate::read_file_into_lines;
use std::ops::Index;

pub fn solve() {
    let lines = read_file_into_lines("input1.txt");
    println!("{}", process_lines(lines));
}

fn process_lines(lines: Vec<String>) -> u32 {
    let mut sum: u32 = 0;

    for line in lines {
        let (mut first, mut last): (String, String) = (String::new(), String::new());

        for i in 0..line.len() {
            let character = line.chars().nth(i).unwrap();
            if character.is_numeric() {
                match first.is_empty() {
                    true => first = character.to_string(),
                    false => last = character.to_string(),
                }
                continue;
            }

            for j in 3..6 {
                if i + j <= line.len() {
                    match get_matching_number(line.index(i..i + j)) {
                        Some(n) => {
                            match first.is_empty() {
                                true => first = u8::to_string(&n),
                                false => last = u8::to_string(&n),
                            }
                            break;
                        }
                        None => {}
                    }
                }
            }
        }

        if last.is_empty() {
            last = first.clone();
        }
        sum += (first.to_string() + last.to_string().as_str()).parse::<u32>().unwrap();
    }

    return sum;
}

fn get_matching_number(string: &str) -> Option<u8> {
    return match string.len() {
        3 => {
            return match string {
                "one" => Option::Some(1),
                "two" => Option::Some(2),
                "six" => Option::Some(6),
                _ => Option::None,
            };
        }
        4 => {
            return match string {
                "four" => Option::Some(4),
                "five" => Option::Some(5),
                "nine" => Option::Some(9),
                _ => Option::None,
            };
        }
        _ => {
            return match string {
                "three" => Option::Some(3),
                "seven" => Option::Some(7),
                "eight" => Option::Some(8),
                _ => Option::None,
            };
        }
    };
}
