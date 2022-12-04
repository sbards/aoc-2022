use crate::{DaySolution, FromInput};

pub struct Day3(Vec<String>);

impl FromInput for Day3 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(lines.collect())
    }
}

impl DaySolution for Day3 {
    fn part_one(&self) -> String {
        self.0
            .iter()
            .map(|rucksack| {
                let (front_compartment, back_compartment) = rucksack.split_at(rucksack.len() / 2);
                item_priority(
                    front_compartment
                        .chars()
                        .into_iter()
                        .find(|item| back_compartment.contains(*item))
                        .unwrap(),
                )
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.0
            .chunks(3)
            .map(|elves| {
                item_priority(
                    elves[0]
                        .chars()
                        .into_iter()
                        .find(|item| elves[1].contains(*item) && elves[2].contains(*item))
                        .unwrap(),
                )
            })
            .sum::<usize>()
            .to_string()
    }
}

fn item_priority(item: char) -> usize {
    match item {
        'a'..='z' => 1 + (item as usize) - ('a' as usize),
        'A'..='Z' => 27 + (item as usize) - ('A' as usize),
        _ => panic!("Invalid item provided"),
    }
}
