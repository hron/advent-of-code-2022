type Calories = u32;
type CaloriesPerElf = Vec<Vec<Calories>>;

fn biggest_elf(calories_per_elf: &CaloriesPerElf) -> Calories {
    calories_per_elf
        .into_iter()
        .map(|elf_calories| elf_calories.into_iter().sum())
        .reduce(|accum, item| if accum >= item { accum } else { item })
        .unwrap()
}

fn parse(input: &str) -> CaloriesPerElf {
    input
        .split("\n\n")
        .map(|elf_calories| {
            elf_calories
                .split("\n")
                .map(|calories: &str| calories.parse::<Calories>().unwrap())
                .collect()
        })
        .collect()
}

fn part_01(input: &str) -> Calories {
    let calories_per_elf = parse(input);
    biggest_elf(&calories_per_elf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "\
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

        assert_eq!(part_01(input), 24000);
    }
}
