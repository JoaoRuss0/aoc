use crate::day3part1::{find_last_digit_j, MATRIX_WIDTH};
use crate::read_file_into_lines;
use std::collections::HashMap;

pub fn solve() {
    let lines = read_file_into_lines("input3.txt");
    let matrix: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect::<Vec<char>>()).collect();
    println!("{}", process_matrix(matrix));
}

fn process_matrix(matrix: Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    let mut gears_map: HashMap<String, Vec<u32>> = HashMap::new();

    for i in 0..matrix.len() {
        let mut j = 0;
        while j < MATRIX_WIDTH {
            if matrix[i][j].is_numeric() {
                let mut number: u32 = 0;
                let last_digit_j = find_last_digit_j(&matrix, i, j, &mut number);
                add_to_bordering_gears(&matrix, i, j, last_digit_j, &mut gears_map, number);
                j = last_digit_j;
            }

            j += 1;
        }
    }

    for key in gears_map.keys() {
        let bordering_numbers = gears_map.get(key).unwrap();

        if bordering_numbers.len() == 2 {
            sum += bordering_numbers[0] * bordering_numbers[1];
        }
    }
    return sum;
}

fn add_to_bordering_gears(
    matrix: &Vec<Vec<char>>,
    current_i: usize,
    first_digit_j: usize,
    last_digit_j: usize,
    gears_map: &mut HashMap<String, Vec<u32>>,
    number: u32,
) {
    let has_previous_j: bool = first_digit_j > 0;
    let has_next_j: bool = last_digit_j < MATRIX_WIDTH - 1;
    let has_top_i: bool = current_i > 0;
    let has_bottom_i: bool = current_i < matrix.len() - 1;

    let start_j = if has_previous_j {
        first_digit_j - 1
    } else {
        first_digit_j
    };
    let end_j = if has_next_j { last_digit_j + 2 } else { last_digit_j + 1 };

    if has_top_i {
        for top_j in start_j..end_j {
            if is_gear(matrix[current_i - 1][top_j]) {
                insert_into_gears_map(gears_map, current_i - 1, top_j, number);
            }
        }
    }

    if has_bottom_i {
        for top_j in start_j..end_j {
            if is_gear(matrix[current_i + 1][top_j]) {
                insert_into_gears_map(gears_map, current_i + 1, top_j, number);
            }
        }
    }

    if has_previous_j && is_gear(matrix[current_i][first_digit_j - 1]) {
        insert_into_gears_map(gears_map, current_i, first_digit_j - 1, number);
    }

    if has_next_j && is_gear(matrix[current_i][last_digit_j + 1]) {
        insert_into_gears_map(gears_map, current_i, last_digit_j + 1, number);
    }
}

fn is_gear(character: char) -> bool {
    return character == '*';
}

fn insert_into_gears_map(gears_map: &mut HashMap<String, Vec<u32>>, i: usize, j: usize, number: u32) {
    let index = format!("{:03}", i) + format!("{:03}", j).as_str();

    if gears_map.get_mut(index.as_str()).is_none() {
        let mut new_array = Vec::new();
        new_array.push(number);
        gears_map.insert(index, new_array);
        return;
    }
    gears_map.get_mut(index.as_str()).unwrap().push(number);
}
