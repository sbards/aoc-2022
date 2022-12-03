use crate::{DaySolution, FromInput};

pub struct Day1(Vec<Option<usize>>);

impl FromInput for Day1 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(lines.map(|line| line.parse().ok()).collect())
    }
}

impl DaySolution for Day1 {
    fn part_one(&self) -> String {
        self.group_values()
            .max()
            .expect("unable to find max of groups")
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut groups: Vec<usize> = self.group_values().collect();
        groups.sort_by(|a, b| b.cmp(a));
        groups[0..3].iter().sum::<usize>().to_string()
    }
}

impl Day1 {
    fn group_values(&self) -> Box<dyn Iterator<Item = usize>> {
        // unstable
        // self.values.group_by(|a, b| a.is_some());

        let mut elves: Vec<usize> = Vec::new();
        let groups = self.0.split(|value| value.is_none());
        for group in groups {
            elves.push(group.iter().map(|value| value.unwrap()).sum());
        }

        Box::new(elves.into_iter())
    }
}
