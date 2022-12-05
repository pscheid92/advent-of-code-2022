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

    println!("solution: {}", count_overlapping_pairs(&pairs));

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

    fn overlap(&self, other: &Self) -> bool {
        other.start <= self.end && self.start <= other.end
    }

    fn parse(s: &str) -> nom::IResult<&str, AssignmentSection> {
        let parser = separated_pair(u64, char('-'), u64);
        let mut parser = map(parser, |(x, y): (u64, u64)| AssignmentSection { start: x, end: y });
        parser(s)
    }
}

struct SectionPair(AssignmentSection, AssignmentSection);

impl SectionPair {
    fn overlap(&self) -> bool {
        let x = self.0.overlap(&self.1);
        let y = self.1.overlap(&self.0);
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

fn count_overlapping_pairs(pairs: &Vec<SectionPair>) -> usize {
    pairs
        .iter()
        .filter(|p| p.overlap())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_overlap_works() {
        let p1 = SectionPair::from_str("2-4,6-8").unwrap();
        let p2 = SectionPair::from_str("2-3,4-5").unwrap();
        assert!(!p1.overlap());
        assert!(!p2.overlap());

        let p3 = SectionPair::from_str("5-7,7-9").unwrap();
        let p4 = SectionPair::from_str("2-8,3-7").unwrap();
        let p5 = SectionPair::from_str("6-6,4-6").unwrap();
        let p6 = SectionPair::from_str("2-6,4-8").unwrap();
        assert!(p3.overlap());
        assert!(p4.overlap());
        assert!(p5.overlap());
        assert!(p6.overlap());
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
    fn count_non_overlapping_works() {
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

        assert_eq!(count_overlapping_pairs(&pairs), 4);
    }
}
