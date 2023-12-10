use crate::{quick_sort, read_file_into_lines};

pub fn solve() {
    let lines = read_file_into_lines("input4.txt");
    println!("{}", process_lines(lines));
}

fn process_lines(lines: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in lines {
        let mut winning_numbers: Vec<u8> = vec![];
        let mut my_numbers: Vec<u8> = vec![];
        get_numbers_from_line(line, &mut my_numbers, &mut winning_numbers);

        let matching_number_count = get_matching_number_count(my_numbers, winning_numbers);

        sum += match matching_number_count {
            0 => 0,
            1 => 1,
            _ => u32::pow(2, matching_number_count - 1),
        };
    }

    return sum;
}

pub fn get_matching_number_count(my_numbers: Vec<u8>, winning_numbers: Vec<u8>) -> u32 {
    let mut matching_number_amount: u32 = 0;
    let mut j = 0;

    for i in 0..my_numbers.len() {
        if j >= winning_numbers.len() {
            break;
        }

        if my_numbers[i] == winning_numbers[j] {
            matching_number_amount += 1;
            j += 1;
        }

        while j + 1 < winning_numbers.len() && my_numbers[i] > winning_numbers[j] {
            j += 1;
            if my_numbers[i] == winning_numbers[j] {
                matching_number_amount += 1;
                j += 1;
            }
        }
    }

    return matching_number_amount;
}

pub fn get_numbers_from_line(line: String, my_numbers: &mut Vec<u8>, winning_numbers: &mut Vec<u8>) {
    let split_line = line.split('|').collect::<Vec<&str>>();

    let my_numbers_string = split_line.get(1).unwrap().trim();
    *my_numbers = get_numbers_from_string(my_numbers_string);
    quick_sort(my_numbers, 0, my_numbers.len() as isize - 1);

    let winning_numbers_string = split_line.get(0).unwrap().split(':').last().unwrap().trim();
    *winning_numbers = get_numbers_from_string(winning_numbers_string);
    quick_sort(winning_numbers, 0, winning_numbers.len() as isize - 1);
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
