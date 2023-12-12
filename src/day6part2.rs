use crate::day6part1::does_new_held_time_beat_record;
use crate::read_file_into_lines;
use rayon::prelude::*;

pub fn solve() {
    let lines = read_file_into_lines("input6.txt");
    println!("{}", process_lines(lines));
}

fn process_lines(lines: Vec<String>) -> u32 {
    let mut times_beaten = 1;

    let time = get_u64_number_from_string_with_colon(lines[0].as_str());
    let distance = get_u64_number_from_string_with_colon(lines[1].as_str());

    let different_tries = if time % 2 == 0 { time / 2 } else { time / 2 };
    times_beaten = (1..different_tries)
        .into_par_iter()
        .map(|to_race_time| {
            if does_new_held_time_beat_record(time, to_race_time, distance) {
                return 1;
            }
            return 0;
        })
        .sum::<u32>()
        * 2;

    if does_new_held_time_beat_record(time, different_tries, distance) {
        if time % 2 == 0 {
            times_beaten += 1;
        } else {
            times_beaten += 2;
        }
    }
    return times_beaten;
}

fn get_u64_number_from_string_with_colon(string: &str) -> u64 {
    let numbers_string = *string.split(':').collect::<Vec<&str>>().get(1).unwrap();
    return numbers_string.replace(' ', "").parse::<u64>().unwrap();
}
