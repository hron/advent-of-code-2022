use super::rucksack::{priority, Rucksack};
use std::ptr;

struct Group<'a> {
    rucksacks: &'a [Rucksack],
}

impl<'a> Group<'a> {
    fn common_item(&self) -> &char {
        let smaller_rucksack: &Rucksack = self
            .rucksacks
            .iter()
            .reduce(|smaller, candidate| {
                if smaller.len() < candidate.len() {
                    smaller
                } else {
                    candidate
                }
            })
            .expect("Cannot find smaller rucksack");
        let other_rucksacks: Vec<&Rucksack> = self
            .rucksacks
            .into_iter()
            .filter(|rucksack| !ptr::eq(*rucksack, smaller_rucksack))
            .collect();
        smaller_rucksack
            .all_items()
            .find(|item| other_rucksacks.iter().all(|r| r.include(item)))
            .expect("Cannot common item for rucksacks")
    }
}

pub fn part_02(input: &str) -> u32 {
    input
        .trim_end()
        .split("\n")
        .map(Rucksack::parse)
        .collect::<Vec<Rucksack>>()
        .chunks_exact(3)
        .map(|chunk| Group { rucksacks: chunk })
        .map(|group| priority(group.common_item()))
        .sum()
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
    fn it_works_part_02() {
        assert_eq!(part_02(TEST_INPUT), 70);
    }
}
