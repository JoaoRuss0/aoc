mod day2part1;
mod day2part2;
mod day4part1;
mod day4part2;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
   day4part2::solve()
}

fn read_file(file_name: &str) -> Vec<String> {

    let file = File::open(file_name).expect("Can not open file");
    let buf_reader = BufReader::new(file);
    return buf_reader.lines().map(|l| l.expect("can not parse line"))
        .collect();
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
            pivot -=1;
        }
        i-=1;
    }

    quick_sort(numbers, start, pivot as isize - 1);
    quick_sort(numbers, pivot + 1, end.clone());
}

fn move_to_new_position(numbers: &mut Vec<u8>, mut index: usize, goal: usize) {

    while index < goal {

        let stored_number = numbers[index];
        numbers[index] = numbers[index +1];
        numbers[index +1] = stored_number;
        index +=1;
    }
}