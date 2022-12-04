use advent_of_code_2022::day_01;
use std::env;
use std::fs;

fn main() {
    let puzzle_input_path = env::args()
        .nth(1)
        .expect("Provide path to file as first argument, e.g. aoc2022 /path/to/puzzle/input.txt");
    let puzzle_input = fs::read_to_string(puzzle_input_path).expect("Cannot read the file");

    println!("Biggest elf is {}", day_01::part_01(&puzzle_input));
}
