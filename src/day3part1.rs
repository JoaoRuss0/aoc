use crate::{read_file};

const MATRIX_WIDTH : usize = 140;

pub fn solve() {

    let lines = read_file("input3.txt");
    let matrix : Vec<Vec<char>> = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    println!("{}", process_matrix(matrix));
}

fn process_matrix(matrix: Vec<Vec<char>>) -> u32 {

    let mut sum = 0;
    for i in 0..matrix.len() {

        let mut j = 0;
        while j < MATRIX_WIDTH {
            if matrix[i][j].is_numeric() {

                let mut number: u32 = 0;
                let last_digit_j = find_last_digit_j(&matrix, i, j, &mut number);

                if has_bordering_symbols(&matrix, i, j, last_digit_j) {
                    sum += number;
                }
                j = last_digit_j;
            }
            j+=1;
        }
    }
    return sum;
}

fn find_last_digit_j(matrix : &Vec<Vec<char>>, current_i : usize, starting_j: usize, number: &mut u32) -> usize {

    let mut last_digit_j: usize = starting_j;
    let mut number_string = matrix[current_i][starting_j].to_string();

    for k in starting_j+1..MATRIX_WIDTH {

        if matrix[current_i][k].is_numeric() {
            last_digit_j = k;
            number_string += matrix[current_i][k].to_string().as_str();
            continue;
        }
        break;
    }

    *number = number_string.parse().unwrap();
    return last_digit_j;
}

fn has_bordering_symbols(matrix : &Vec<Vec<char>>, current_i : usize, first_digit_j: usize, last_digit_j: usize) -> bool {

    let has_previous_j: bool = first_digit_j > 0;
    let has_next_j: bool = last_digit_j < MATRIX_WIDTH - 1;
    let has_top_i: bool = current_i > 0;
    let has_bottom_i: bool = current_i < matrix.len() - 1;

    let start_j = if has_previous_j { first_digit_j - 1 } else { first_digit_j };
    let end_j = if has_next_j { last_digit_j + 2 } else { last_digit_j + 1};

    if has_top_i {
        for top_j in start_j..end_j {
            if is_symbol(matrix[current_i - 1][top_j]) {
                return true;
            }
        }
    }

    if has_bottom_i {
        for top_j in start_j..end_j {
            if is_symbol(matrix[current_i + 1][top_j]) {
                return true;
            }
        }
    }

    if has_previous_j && is_symbol(matrix[current_i][first_digit_j - 1]) {
        return true;
    }
    return has_next_j && is_symbol(matrix[current_i][last_digit_j + 1]);
}

fn is_symbol(character: char) -> bool {

    return !character.is_numeric() && character != '.';
}