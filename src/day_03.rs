#[derive(Debug)]
struct Rucksack {
    compartments: [Vec<char>; 2],
}

impl Rucksack {
    fn parse(raw_rucksack: &str) -> Rucksack {
        let middle = raw_rucksack.len() / 2;
        Rucksack {
            compartments: [
                raw_rucksack[..middle].chars().collect(),
                raw_rucksack[middle..].chars().collect(),
            ],
        }
    }

    fn misplaced_item(&self) -> &char {
        self.compartments[0]
            .iter()
            .find(|item| self.compartments[1].contains(item))
            .expect(&format!(
                "No misplaced tiem is found compartments {:#?}",
                self.compartments
            ))
    }

    fn priority(&self) -> usize {
        let mut priority = 1;
        let misplaced_item = self.misplaced_item();
        for (i, c) in (b'a'..=b'z').chain(b'A'..=b'Z').enumerate() {
            if c == *misplaced_item as u8 {
                priority = priority + i;
            }
        }
        priority
    }
}

pub fn part_01(input: &str) -> usize {
    let rucksacks: Vec<Rucksack> = input.trim_end().split("\n").map(Rucksack::parse).collect();
    rucksacks.iter().map(|r| r.priority()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn it_works_part_01() {
        assert_eq!(part_01(TEST_INPUT), 157);
    }
}
