use super::rucksack::Rucksack;

pub fn part_01(input: &str) -> u32 {
    let rucksacks: Vec<Rucksack> = input.trim_end().split("\n").map(Rucksack::parse).collect();
    rucksacks
        .iter()
        .map(|r| r.priority(r.misplaced_item()))
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
    fn it_works_part_01() {
        assert_eq!(part_01(TEST_INPUT), 157);
    }
}
