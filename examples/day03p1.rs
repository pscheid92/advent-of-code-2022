extern crate aoc2022;

use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use nom::character::complete::alpha1;
use nom::combinator::{eof, map_res};
use nom::IResult;
use nom::sequence::terminated;

use aoc2022::*;

fn main() -> Result<()> {
    let lines = read(3).context("error reading input")?;
    let rucksacks = lines
        .iter()
        .map(|l| Rucksack::from_str(l))
        .collect::<Result<Vec<_>, _>>()
        .context("error parsing rucksacks")?;

    let result = process_multiple_rucksacks(&rucksacks).context("error processing rucksacks")?;
    println!("solution: {}", result);
    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct Rucksack {
    first: String,
    second: String,
}

impl Rucksack {
    fn new(first: &str, second: &str) -> Self {
        Rucksack {
            first: first.to_string(),
            second: second.to_string(),
        }
    }

    fn parse(s: &str) -> IResult<&str, Rucksack> {
        let parser = terminated(alpha1, eof);
        let mut parser = map_res(parser, |l: &str| {
            if l.len() % 2 != 0 {
                return Err(anyhow!("uneven characters not allowed"));
            }

            let (first, second) = s.split_at(l.len() / 2);
            Ok(Rucksack::new(first, second))
        });

        parser(s)
    }
}

fn find_error(rucksack: &Rucksack) -> char {
    let mut hashset = HashSet::new();

    for i in rucksack.first.chars() {
        hashset.insert(i);
    }

    for j in rucksack.second.chars() {
        if hashset.contains(&j) {
            return j;
        }
    }

    // There always is a duplicate!
    unreachable!()
}

fn get_priority(c: char) -> Result<u32> {
    match c {
        'a'..='z' => Ok((c as u32) - ('a' as u32) + 1),
        'A'..='Z' => Ok((c as u32) - ('A' as u32) + 27),
        _ => Err(anyhow!("invalid rucksack item")),
    }
}

fn process_multiple_rucksacks(rucksacks: &Vec<Rucksack>) -> Result<u32> {
    let priority = rucksacks
        .iter()
        .map(|r| find_error(r))
        .map(|e| get_priority(e))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    Ok(priority)
}

impl_from_str!(Rucksack);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_in_rucksack_description() {
        let case1 = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let expect1 = Rucksack { first: "vJrwpWtwJgWr".to_string(), second: "hcsFMMfFFhFp".to_string() };
        assert_eq!(Rucksack::from_str(case1).unwrap(), expect1);

        let case2 = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let expect2 = Rucksack { first: "jqHRNqRjqzjGDLGL".to_string(), second: "rsFMfFZSrLrFZsSL".to_string() };
        assert_eq!(Rucksack::from_str(case2).unwrap(), expect2);

        assert!(Rucksack::from_str("").is_err());
        assert!(Rucksack::from_str("a").is_err());
        assert!(Rucksack::from_str("abc").is_err());
    }

    #[test]
    fn check_rucksack_for_duplicate_items() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        let expected = vec!['p', 'L', 'P', 'v', 't', 's'];
        let rucksacks = lines.iter().map(|l| Rucksack::from_str(l).unwrap()).collect::<Vec<_>>();

        for i in 0..expected.len() {
            let r = rucksacks.get(i).unwrap();
            let e = expected.get(i).unwrap().clone();
            assert_eq!(find_error(r), e);
        }
    }

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
    fn check_all_rucksacks() {
        let rucksacks = vec!["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"]
            .iter()
            .map(|l| Rucksack::from_str(l).unwrap())
            .collect::<Vec<_>>();

        let priority = process_multiple_rucksacks(&rucksacks);
        assert_eq!(priority.unwrap(), 157);
    }
}
