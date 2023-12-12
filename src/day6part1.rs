use crate::{get_numbers_from_string_with_colon, read_file_into_lines};

pub fn solve() {
    let lines = read_file_into_lines("input6.txt");
    println!("{}", process_lines(lines));
}

fn process_lines(lines: Vec<String>) -> u64 {
    let mut multiplication = 1;

    let times = get_numbers_from_string_with_colon(lines[0].as_str());
    let distances = get_u16_numbers_from_string_with_colon(lines[1].as_str());

    for i in 0..times.len() {
        let mut times_beaten = 0;
        let different_tries = times[i] / 2;

        for j in 1..different_tries {
            if does_new_held_time_beat_record(u64::from(times[i]), j as u64, distances[i] as u64) {
                times_beaten += 1;
            }
        }
        times_beaten *= 2;

        if does_new_held_time_beat_record(u64::from(times[i]), different_tries as u64, distances[i] as u64) {
            if times[i] % 2 == 0 {
                times_beaten += 1;
            } else {
                times_beaten += 2;
            }
        }

        if times_beaten != 0 {
            multiplication *= times_beaten;
        }
    }
    return multiplication;
}

fn get_u16_numbers_from_string_with_colon(string: &str) -> Vec<u16> {
    let numbers_string = *string.split(':').collect::<Vec<&str>>().get(1).unwrap();
    return numbers_string
        .split(' ')
        .filter(|&numer| !numer.is_empty())
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| n.parse::<u16>().expect("Could not parse into number"))
        .collect::<Vec<u16>>();
}

pub fn does_new_held_time_beat_record(total_time: u64, to_race_time: u64, goal_distance: u64) -> bool {
    return (total_time - to_race_time) * to_race_time > goal_distance;
}
