#[derive(Debug)]
pub struct Rucksack {
    compartments: [Vec<char>; 2],
}

impl Rucksack {
    pub fn parse(raw_rucksack: &str) -> Rucksack {
        let middle = raw_rucksack.len() / 2;
        Rucksack {
            compartments: [
                raw_rucksack[..middle].chars().collect(),
                raw_rucksack[middle..].chars().collect(),
            ],
        }
    }

    pub fn misplaced_item(&self) -> &char {
        self.compartments[0]
            .iter()
            .find(|item| self.compartments[1].contains(item))
            .expect(&format!(
                "No misplaced item is found compartments {:#?}",
                self.compartments
            ))
    }

    pub fn len(&self) -> usize {
        self.compartments.iter().map(|c| c.len()).sum()
    }

    pub fn all_items(&self) -> std::iter::Chain<std::slice::Iter<char>, std::slice::Iter<char>> {
        self.compartments[0]
            .iter()
            .chain(self.compartments[1].iter())
            .into_iter()
    }

    pub(crate) fn include(&self, item: &char) -> bool {
        self.all_items().any(|c| c == item)
    }
}

pub fn priority(item: &char) -> u32 {
    let mut priority: u32 = 1;
    for (i, c) in (b'a'..=b'z').chain(b'A'..=b'Z').enumerate() {
        if c == *item as u8 {
            priority = priority + i as u32;
        }
    }
    priority
}
