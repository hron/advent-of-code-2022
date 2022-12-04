type Snack = u32;
type SnacksPerElf = Vec<Vec<Snack>>;

fn total_calories_for_top(snacks_per_elf: &SnacksPerElf, n: usize) -> Snack {
    let mut top_n_elfs: Vec<Snack> = vec![0; n];

    let total_calories_per_elf = snacks_per_elf
        .into_iter()
        .map(|elf_calories| elf_calories.into_iter().sum());

    for (_, c) in total_calories_per_elf.enumerate() {
        top_n_elfs.push(c);
        top_n_elfs.sort();
        top_n_elfs.remove(0);
    }
    top_n_elfs.into_iter().sum()
}

fn parse(input: &str) -> SnacksPerElf {
    input
        .trim_end()
        .split("\n\n")
        .map(|elf_calories| {
            elf_calories
                .split("\n")
                .map(|calories: &str| {
                    calories
                        .parse::<Snack>()
                        .expect(&format!("Cannot parse '{calories}'"))
                })
                .collect()
        })
        .collect()
}

pub fn part_01(input: &str) -> Snack {
    let snacks_per_elf = parse(input);
    total_calories_for_top(&snacks_per_elf, 1)
}

pub fn part_02(input: &str) -> Snack {
    let snacks_per_elf = parse(input);
    total_calories_for_top(&snacks_per_elf, 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn it_works_part_01() {
        assert_eq!(part_01(TEST_INPUT), 24000);
    }

    #[test]
    fn it_works_part_02() {
        assert_eq!(part_02(TEST_INPUT), 45000);
    }
}
