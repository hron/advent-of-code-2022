use advent_of_code_2022::day_01;
use advent_of_code_2022::day_02;
use advent_of_code_2022::day_03;

fn main() {
    let config = advent_of_code_2022::get_config();

    let answer = match config.day.as_str() {
        "day01p01" => day_01::part_01(&config.puzzle_input),
        "day01p02" => day_01::part_02(&config.puzzle_input),
        "day02p01" => day_02::part_01(&config.puzzle_input),
        "day02p02" => day_02::part_02(&config.puzzle_input),
        "day03p01" => day_03::part_01(&config.puzzle_input) as u32,
        &_ => panic!("Unknown day"),
    };

    println!("Answer {}", answer);
}
