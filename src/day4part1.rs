use crate::{read_file, quick_sort};

pub fn solve() {

    let lines = read_file("input4.txt");
    println!("{}", process_lines(lines));
}

fn process_lines(lines: Vec<String>) -> u32 {

    let mut sum: u32 = 0;
    for line in lines {

        let split_line = line.split('|').collect::<Vec<&str>>();

        let my_numbers_string = split_line.get(1)
            .unwrap()
            .trim();
        let mut my_numbers = get_numbers_from_string(my_numbers_string);
        let my_numbers_amount = my_numbers.len() as isize;
        quick_sort(&mut my_numbers, 0, my_numbers_amount - 1);

        let winning_numbers_string = split_line.get(0)
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .trim();
        let mut winning_numbers = get_numbers_from_string(winning_numbers_string);
        let winning_numbers_amount = winning_numbers.len() as isize;
        quick_sort(&mut winning_numbers, 0, winning_numbers_amount - 1);

        let mut matching_number_amount : u32 = 0;
        let mut j = 0;
        for i in 0..my_numbers_amount as usize {

            if j >= winning_numbers.len() {
                break;
            }

            if my_numbers[i] == winning_numbers[j] {
                matching_number_amount += 1;
                j+=1;
            }

            while j + 1 < winning_numbers.len() && my_numbers[i] > winning_numbers[j] {
                j+=1;
                if(my_numbers[i] == winning_numbers[j]) {
                    matching_number_amount +=1;
                    j+=1;
                }
            }
        }

        sum += match matching_number_amount {
            0 => 0,
            1 => 1,
            _ => u32::pow(2, matching_number_amount - 1)
        };
    }

    return sum;
}

fn get_numbers_from_string(string: &str) -> Vec<u8> {

        return string
        .split(' ')
        .filter(|&numer| !numer.is_empty())
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| n.parse::<u8>().expect("Could not parse into number"))
        .collect::<Vec<u8>>()
}