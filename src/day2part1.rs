use crate::read_file;

struct ColourCount {
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

pub fn solve() {
    let lines = read_file("input2.txt");
    println!("{}", get_sum_of_possible_games(lines));
}

fn get_sum_of_possible_games(lines: Vec<String>) -> u16 {
    let mut sum: u16 = 0;

    for line in lines {
        let split_line = line.split(':').collect::<Vec<&str>>();

        let game_id_string: &str = split_line.get(0).unwrap();
        let game_id: u8 = game_id_string
            .split(' ')
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .parse()
            .unwrap();

        let color_count: ColourCount = get_max_color_count(split_line.get(1).unwrap().trim());

        if color_count.max_red <= 12 && color_count.max_green <= 13 && color_count.max_blue <= 14 {
            sum += game_id as u16;
        }
    }
    return sum;
}

fn get_max_color_count(sets_string: &str) -> ColourCount {
    let mut color_count = ColourCount {
        max_blue: 0,
        max_green: 0,
        max_red: 0,
    };

    for set in sets_string.split(';') {
        for color_string in set.split(',') {
            let color_split = color_string.trim().split(' ').collect::<Vec<&str>>();

            let count_string = color_split.get(0).unwrap();
            let count: u32 = count_string.to_string().parse::<u32>().unwrap();
            let color = *color_split.get(1).unwrap();

            match color {
                "red" => {
                    if color_count.max_red < count {
                        color_count.max_red = count;
                    }
                }
                "green" => {
                    if color_count.max_green < count {
                        color_count.max_green = count;
                    }
                }
                _ => {
                    if color_count.max_blue < count {
                        color_count.max_blue = count;
                    }
                }
            }
        }
    }
    return color_count;
}
