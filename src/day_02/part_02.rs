#[derive(PartialEq, Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn parse(raw_play: &str) -> Play {
        if raw_play == "A" || raw_play == "X" {
            Self::Rock
        } else if raw_play == "B" || raw_play == "Y" {
            Self::Paper
        } else if raw_play == "C" || raw_play == "Z" {
            Self::Scissors
        } else {
            panic!("Unknown Play: '{}'", raw_play);
        }
    }
}

#[derive(Debug)]
enum RoundResult {
    Lose,
    Draw,
    Win,
}

impl RoundResult {
    fn parse(raw_round_result: &str) -> RoundResult {
        match raw_round_result {
            "X" => RoundResult::Lose,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            &_ => panic!("Invalid round result: '{}'", raw_round_result),
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Play,
    result: RoundResult,
}

impl Round {
    fn parse(raw_round: &str) -> Round {
        let round: Vec<&str> = raw_round.split(" ").collect();
        Self {
            opponent: Play::parse(round[0]),
            result: RoundResult::parse(round[1]),
        }
    }

    fn victory_score(&self) -> u32 {
        match self.result {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Lose => 0,
        }
    }

    fn shape_score(&self) -> u32 {
        if self.my_shape() == Play::Rock {
            1
        } else if self.my_shape() == Play::Paper {
            2
        } else {
            3
        }
    }

    fn total_score(&self) -> u32 {
        self.victory_score() + self.shape_score()
    }

    fn my_shape(&self) -> Play {
        match self.result {
            RoundResult::Win => self.winning_shape(),
            RoundResult::Draw => self.shape_for_draw(),
            RoundResult::Lose => self.loosing_shape(),
        }
    }

    fn winning_shape(&self) -> Play {
        match self.opponent {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }

    fn shape_for_draw(&self) -> Play {
        match self.opponent {
            Play::Rock => Play::Rock,
            Play::Paper => Play::Paper,
            Play::Scissors => Play::Scissors,
        }
    }

    fn loosing_shape(&self) -> Play {
        match self.opponent {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }
}

struct GameSeries {
    rounds: Vec<Round>,
}

impl GameSeries {
    fn parse(input: &str) -> GameSeries {
        Self {
            rounds: input.trim_end().split("\n").map(Round::parse).collect(),
        }
    }

    fn total_score(&self) -> u32 {
        self.rounds.iter().map(|r| r.total_score()).sum()
    }
}

pub fn part_02(input: &str) -> u32 {
    GameSeries::parse(input).total_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
A Y
B X
C Z
";

    #[test]
    fn it_works_part_02() {
        assert_eq!(12, part_02(TEST_INPUT));
    }
}
