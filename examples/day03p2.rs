extern crate aoc2022;
extern crate core;

use anyhow::{Result, Context, anyhow};
use std::collections::HashSet;
use nom::character::complete::alpha1;
use nom::combinator::{eof, map_res};
use nom::IResult;
use nom::sequence::terminated;
use aoc2022::*;

fn main() -> Result<()> {
    let rucksacks = read(3)
        .context("error reading input")?
        .iter()
        .map(|l| Rucksack::new(l))
        .collect::<Vec<_>>();

    let result: u32 = rucksacks
        .chunks_exact(3)
        .map(|x| x.try_into())
        .collect::<Result<Vec<&[Rucksack; 3]>, _>>()?
        .iter()
        .map(|x| process_group(x))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    println!("solution: {}", result);
    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct Rucksack {
    content: String
}

impl Rucksack {
    fn new(content: &str) -> Self {
        Rucksack { content: content.to_string() }
    }
}

fn get_priority(c: char) -> Result<u32> {
    match c {
        'a'..='z' => Ok((c as u32) - ('a' as u32) + 1),
        'A'..='Z' => Ok((c as u32) - ('A' as u32) + 27),
        _ => Err(anyhow!("invalid rucksack item")),
    }
}

fn summarise_rucksacks(rucksack: &Rucksack) -> HashSet<char> {
    let mut set = HashSet::new();
    for c in rucksack.content.chars() {
        set.insert(c);
    }
    set
}

fn identify_badge(rucksacks: &[Rucksack; 3]) -> Option<char> {
    let set1 = summarise_rucksacks(&rucksacks[0]);
    let set2 = summarise_rucksacks(&rucksacks[1]);
    let set3 = summarise_rucksacks(&rucksacks[2]);

    let result = set1.intersection(&set2).cloned().collect::<HashSet<_>>();
    let result = result.intersection(&set3).cloned().collect::<HashSet<_>>();

    result.iter().next().cloned()
}

fn process_group(group: &[Rucksack; 3]) -> Result<u32> {
    let badge = identify_badge(group).context("no badge")?;
    let priority = get_priority(badge).context("invalid badge")?;
    Ok(priority)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_the_priority() {
        assert_eq!(get_priority('p').unwrap(), 16);
        assert_eq!(get_priority('L').unwrap(), 38);
        assert_eq!(get_priority('P').unwrap(), 42);
        assert_eq!(get_priority('v').unwrap(), 22);
        assert_eq!(get_priority('t').unwrap(), 20);
        assert_eq!(get_priority('s').unwrap(), 19);
        assert!(get_priority('0').is_err());
    }

    #[test]
    fn can_identify_badge() {
        let group1 = [
            Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Rucksack::new("PmmdzqPrVvPwwTWBwg"),
        ];

        let group2 = [
            Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            Rucksack::new("ttgJtRGJQctTZtZT"),
            Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        assert_eq!(identify_badge(&group1), Some('r'));
        assert_eq!(identify_badge(&group2), Some('Z'));
    }

    #[test]
    fn can_process_a_group() {
        let group1 = [
            Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Rucksack::new("PmmdzqPrVvPwwTWBwg"),
        ];

        let group2 = [
            Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            Rucksack::new("ttgJtRGJQctTZtZT"),
            Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        assert_eq!(process_group(&group1).unwrap(), 18);
        assert_eq!(process_group(&group2).unwrap(), 52);
    }
}
