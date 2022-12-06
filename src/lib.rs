use std::env;
use std::fs;

pub mod day_01;
pub mod day_02;
pub mod day_03;

pub fn get_puzzle_input() -> String {
    let puzzle_input_path = env::args()
        .nth(1)
        .expect("Provide path to file as first argument, e.g. aoc2022 /path/to/puzzle/input.txt");
    fs::read_to_string(puzzle_input_path).expect("Cannot read the file")
}
