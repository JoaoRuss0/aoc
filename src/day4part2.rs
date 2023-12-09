use crate::day4part1::{get_matching_number_count, get_numbers_from_line};
use crate::read_file;

pub fn solve() {
    let lines = read_file("input4.txt");
    println!("{}", process_lines(lines));
}

fn process_lines(lines: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    let mut duplicate_scratchpads = vec![1; lines.len()];

    for i in 0..lines.len() {
        let mut winning_numbers: Vec<u8> = vec![];
        let mut my_numbers: Vec<u8> = vec![];
        get_numbers_from_line(lines[i].to_string(), &mut my_numbers, &mut winning_numbers);

        let matching_number_count = get_matching_number_count(my_numbers, winning_numbers);

        for j in 1..matching_number_count + 1 {
            duplicate_scratchpads[i + j as usize] += duplicate_scratchpads[i];
        }
        sum += duplicate_scratchpads[i];
    }

    return sum;
}
