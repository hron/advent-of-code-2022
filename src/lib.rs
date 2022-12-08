use std::env;
use std::fs;

pub mod day_01;
pub mod day_02;
pub mod day_03;

pub struct Config {
    pub day: String,
    pub puzzle_input: String,
}

const HELP_MESSAGE: &str =
    "Provide path to file as first argument, e.g. day01p01 /path/to/puzzle/input/day1.txt";

pub fn get_config() -> Config {
    let puzzle_input_path = env::args().nth(2).expect(HELP_MESSAGE);

    Config {
        day: env::args().nth(1).expect(HELP_MESSAGE),
        puzzle_input: fs::read_to_string(puzzle_input_path).expect("Cannot read the file"),
    }
}
