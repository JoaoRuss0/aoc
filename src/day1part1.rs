use crate::read_file;

const BASE: u32 = 10;

pub fn solve() {
    let lines = read_file("input1.txt");
    println!("{}", process_lines(lines));
}

fn process_lines(lines: Vec<String>) -> u32 {
    let mut sum: u32 = 0;

    for line in lines {
        let (mut first, mut last): (char, char) = (char::default(), char::default());

        for char in line.chars() {
            if char.is_numeric() {
                if first == char::default() {
                    first = char;
                    continue;
                }
                last = char;
            }
        }

        if last == char::default() {
            last = first;
        }
        sum += (first.to_string() + last.to_string().as_str()).parse::<u32>().unwrap();
    }

    return sum;
}
