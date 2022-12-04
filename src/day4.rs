use crate::{DaySolution, FromInput};

pub struct Day4(Vec<ElfPair>);

impl FromInput for Day4 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(lines.map(|line| ElfPair::from_string(&line)).collect())
    }
}

impl DaySolution for Day4 {
    fn part_one(&self) -> String {
        self.0
            .iter()
            .filter(|elf_pair| elf_pair.contains())
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.0
            .iter()
            .filter(|elf_pair| elf_pair.overlaps())
            .count()
            .to_string()
    }
}

struct ElfPair {
    thing1: SectionPair,
    thing2: SectionPair,
}

impl ElfPair {
    fn contains(&self) -> bool {
        self.thing1.contains(&self.thing2) || self.thing2.contains(&self.thing1)
    }

    fn overlaps(&self) -> bool {
        self.thing1.overlaps(&self.thing2)
    }

    fn from_string(string: &str) -> Self {
        let (first_pair, second_pair) = string
            .split_once(',')
            .expect("elf pair should be split by a ','");
        ElfPair {
            thing1: SectionPair::from_string(first_pair),
            thing2: SectionPair::from_string(second_pair),
        }
    }
}

struct SectionPair {
    start: usize,
    finish: usize,
}

impl SectionPair {
    fn contains(&self, other: &SectionPair) -> bool {
        self.start <= other.start && self.finish >= other.finish
    }

    fn overlaps(&self, other: &SectionPair) -> bool {
        self.start <= other.finish && self.finish >= other.start
    }

    fn from_string(string: &str) -> Self {
        let (start, finish) = string
            .split_once('-')
            .expect("section pair should be split by a '-'");
        SectionPair {
            start: start.parse().expect("Should be a usize"),
            finish: finish.parse().expect("Should be a usize"),
        }
    }
}
