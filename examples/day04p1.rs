use std::str::FromStr;

use anyhow::Context;
use nom::character::complete::{char, u64};
use nom::combinator::{eof, map};
use nom::sequence::{separated_pair, terminated};

use aoc2022::{impl_from_str, read};

fn main() -> anyhow::Result<()> {
    let lines = read(4).context("error reading input")?;

    let pairs = lines
        .iter()
        .map(|l| SectionPair::from_str(l))
        .collect::<Result<Vec<_>, _>>()?;

    println!("solution: {}", count_enclosed_pairs(&pairs));

    Ok(())
}

struct AssignmentSection {
    start: u64,
    end: u64,
}

impl AssignmentSection {
    fn new(start: u64, end: u64) -> Self {
        AssignmentSection { start, end }
    }

    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn parse(s: &str) -> nom::IResult<&str, AssignmentSection> {
        let parser = separated_pair(u64, char('-'), u64);
        let mut parser = map(parser, |(x, y): (u64, u64)| AssignmentSection { start: x, end: y });
        parser(s)
    }
}

struct SectionPair(AssignmentSection, AssignmentSection);

impl SectionPair {
    fn fully_overlap(&self) -> bool {
        let x = self.0.fully_contains(&self.1);
        let y = self.1.fully_contains(&self.0);
        x || y
    }

    fn parse(s: &str) -> nom::IResult<&str, SectionPair> {
        let parser = separated_pair(AssignmentSection::parse, char(','), AssignmentSection::parse);
        let parser = terminated(parser, eof);
        let mut parser = map(parser, |(x, y): (AssignmentSection, AssignmentSection)| SectionPair(x, y));
        parser(s)
    }
}

impl_from_str!(AssignmentSection);
impl_from_str!(SectionPair);

fn count_enclosed_pairs(pairs: &Vec<SectionPair>) -> usize {
    pairs
        .iter()
        .filter(|p| p.fully_overlap())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_contains_works() {
        let section = AssignmentSection::new(5, 10);

        let inner = AssignmentSection::new(6, 8);
        let left_overlap = AssignmentSection::new(1, 8);
        let right_overlap = AssignmentSection::new(6, 11);
        let outer = AssignmentSection::new(0, 15);

        assert!(section.fully_contains(&inner));
        assert!(!inner.fully_contains(&section));

        assert!(!section.fully_contains(&left_overlap));
        assert!(!section.fully_contains(&right_overlap));
        assert!(!section.fully_contains(&outer));
    }

    #[test]
    fn parse_section_pair_works() {
        let pair = SectionPair::from_str("2-4,6-8").unwrap();
        assert_eq!(pair.0.start, 2);
        assert_eq!(pair.0.end, 4);
        assert_eq!(pair.1.start, 6);
        assert_eq!(pair.1.end, 8);
    }

    #[test]
    fn pair_fully_overlap_works() {
        let pair = SectionPair(
            AssignmentSection::new(5, 10),
            AssignmentSection::new(6, 8),
        );
        assert!(pair.fully_overlap());

        let pair_switched = SectionPair(
            AssignmentSection::new(6, 8),
            AssignmentSection::new(5, 10),
        );
        assert!(pair_switched.fully_overlap())
    }

    #[test]
    fn count_enclosed_pairs_works() {
        let lines = vec![
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ];

        let pairs: Vec<SectionPair> = lines
            .iter()
            .map(|l| SectionPair::from_str(l).unwrap())
            .collect();

        assert_eq!(count_enclosed_pairs(&pairs), 2);
    }
}
