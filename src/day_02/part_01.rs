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
struct Round {
    opponent: Play,
    me: Play,
}

impl Round {
    fn parse(raw_round: &str) -> Round {
        let round: Vec<&str> = raw_round.split(" ").collect();
        Self {
            opponent: Play::parse(round[0]),
            me: Play::parse(round[1]),
        }
    }

    fn is_win(&self) -> bool {
        match self.opponent {
            Play::Paper => self.me == Play::Scissors,
            Play::Rock => self.me == Play::Paper,
            Play::Scissors => self.me == Play::Rock,
        }
    }

    fn is_draw(&self) -> bool {
        self.opponent == self.me
    }

    fn victory_score(&self) -> u32 {
        if self.is_win() {
            6
        } else if self.is_draw() {
            3
        } else {
            0
        }
    }

    fn shape_score(&self) -> u32 {
        if self.me == Play::Rock {
            1
        } else if self.me == Play::Paper {
            2
        } else {
            3
        }
    }

    fn total_score(&self) -> u32 {
        self.victory_score() + self.shape_score()
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

pub fn part_01(input: &str) -> u32 {
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
    fn it_works_part_01() {
        assert_eq!(15, part_01(TEST_INPUT));
    }
}
