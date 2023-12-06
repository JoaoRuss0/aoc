mod day2part1;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};


fn main() {
    day2part1::solve()
}

fn read_file(file_name: &str) -> Vec<String> {

    let file = File::open(file_name).expect("Can not open file");
    let buf_reader = BufReader::new(file);
    return buf_reader.lines().map(|l| l.expect("can not parse line"))
        .collect();
}