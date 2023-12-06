use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
use std::collections::HashMap;

struct Game {
    id: u8,
    colors: HashMap<String, u32>
}

fn main() {

    let lines = read_file("input2.txt");
    let games = get_games_from_lines(lines);

    println!("{}", get_sum_of_possible_games(games));
}

fn read_file(file_name: &str) -> Vec<String> {

    let file = File::open(file_name).expect("Can not open file");
    let buf_reader = BufReader::new(file);
    return buf_reader.lines().map(|l| l.expect("can not parse line"))
        .collect();
}

fn get_games_from_lines(lines: Vec<String>) -> Vec<Game>{

    let mut games: Vec<Game> = Vec::with_capacity(lines.len());

    for line in lines {

        let split_line = line.split(':').collect::<Vec<&str>>();
        let game_id_string: &str = split_line.get(0).unwrap();
        let game_id: u8 = game_id_string.split(' ').collect::<Vec<&str>>().get(1).unwrap().parse().unwrap();

        let color_count : HashMap<String, u32> = get_max_color_count(split_line.get(1).unwrap().trim());
        //print!("{} ", game_id);
        //print!("{:?} ", color_count);
        //println!("{}", split_line.get(1).unwrap().trim());
        let game: Game = Game{id: game_id, colors: color_count};
        games.push(game);
    }
    return games;
}

fn get_max_color_count(sets_string: &str) -> HashMap<String, u32>{

    let mut color_count = HashMap::<String, u32>::new();

    for set in sets_string.split(';') {
        for color_string in set.split(',') {

            let color_split = color_string.trim().split(' ').collect::<Vec<&str>>();

            let count_string = color_split.get(0).unwrap();
            let count: u32 = count_string.to_string().parse::<u32>().unwrap();
            let colour = *color_split.get(1).unwrap();

            if !color_count.contains_key(colour) || *color_count.get(colour).unwrap() < count {
                color_count.insert(colour.to_string(), count);
            }
        }
    }
    return color_count;
}

fn get_sum_of_possible_games(games: Vec<Game>) -> u16{

    let mut sum: u16 = 0;
    for game in games {
        if *game.colors.get("red").unwrap() <= 12 && *game.colors.get("green").unwrap() <= 13 && *game.colors.get("blue").unwrap() <= 14 {
            //print!("{} ", game.id);
            sum += game.id as u16;
        }
    }
    return sum;
}