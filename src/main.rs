mod day1part1;
mod day1part2;
mod day2part1;
mod day2part2;
mod day3part1;
mod day3part2;
mod day4part1;
mod day4part2;
mod day5part1;
mod day5part2;
mod day6part1;
mod day6part2;
mod day7part1;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    day7part1::solve()
}

fn read_file_into_lines(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("Can not open file");
    let buf_reader = BufReader::new(file);
    return buf_reader.lines().map(|l| l.expect("can not parse line")).collect();
}

fn read_file_into_chunks(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("Can not open file");

    let mut buf_reader = BufReader::new(file);
    let mut file_string: String = String::new();
    buf_reader.read_to_string(&mut file_string);

    return file_string
        .split("\n\n")
        .map(|chunk| chunk.to_string())
        .collect::<Vec<String>>();
}

fn quick_sort(numbers: &mut Vec<u8>, start: usize, end: isize) {
    if end <= start as isize {
        return;
    }

    let mut i: usize = end as usize;
    let mut pivot = end.clone() as usize;

    while i > start {
        if numbers[i - 1] > numbers[pivot] {
            move_to_new_position(numbers, i - 1, pivot);
            pivot -= 1;
        }
        i -= 1;
    }

    quick_sort(numbers, start, pivot as isize - 1);
    quick_sort(numbers, pivot + 1, end.clone());
}

fn move_to_new_position(numbers: &mut Vec<u8>, mut index: usize, goal: usize) {
    while index < goal {
        let stored_number = numbers[index];
        numbers[index] = numbers[index + 1];
        numbers[index + 1] = stored_number;
        index += 1;
    }
}

fn get_numbers_from_string_with_colon(string: &str) -> Vec<u8> {
    let numbers_string = *string.split(':').collect::<Vec<&str>>().get(1).unwrap();
    return get_numbers_from_string(numbers_string);
}

fn get_numbers_from_string(string: &str) -> Vec<u8> {
    return string
        .split(' ')
        .filter(|&numer| !numer.is_empty())
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| n.parse::<u8>().expect("Could not parse into number"))
        .collect::<Vec<u8>>();
}
