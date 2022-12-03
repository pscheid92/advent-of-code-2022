extern crate aoc2022;

use std::collections::HashSet;
use std::str::FromStr;
use aoc2022::*;

fn main() {
    let lines = read(3).unwrap();
    let rucksacks = lines
        .iter()
        .map(|l| Rucksack::from_str(l))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let result = process_multiple_rucksacks(&rucksacks).unwrap();
    println!("solution: {}", result);
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
}

impl FromStr for Rucksack {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = s.len();

        if (n == 0) || (n % 2 != 0) {
            return Err("");
        }

        let (first, second) = s.split_at(n / 2);
        let rucksack = Rucksack::new(first, second);
        Ok(rucksack)
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

    unreachable!()
}

fn get_priority(c: char) -> Result<u32, &'static str> {
    match c {
        'a'..='z' => Ok((c as u32) - ('a' as u32) + 1),
        'A'..='Z' => Ok((c as u32) - ('A' as u32) + 27),
        _ => Err(""),
    }
}

fn process_multiple_rucksacks(rucksacks: &Vec<Rucksack>) -> Result<u32, &'static str> {
    let priority = rucksacks
        .iter()
        .map(|r| find_error(r))
        .map(|e| get_priority(e))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    Ok(priority)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_in_rucksack_description() {
        let case1 = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let expect1 = Rucksack { first: "vJrwpWtwJgWr".to_string(), second: "hcsFMMfFFhFp".to_string() };
        assert_eq!(Rucksack::from_str(case1), Ok(expect1));

        let case2 = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let expect2 = Rucksack { first: "jqHRNqRjqzjGDLGL".to_string(), second: "rsFMfFZSrLrFZsSL".to_string() };
        assert_eq!(Rucksack::from_str(case2), Ok(expect2));

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
        assert_eq!(get_priority('p'), Ok(16));
        assert_eq!(get_priority('L'), Ok(38));
        assert_eq!(get_priority('P'), Ok(42));
        assert_eq!(get_priority('v'), Ok(22));
        assert_eq!(get_priority('t'), Ok(20));
        assert_eq!(get_priority('s'), Ok(19));
        assert!(get_priority('0').is_err());
    }

    #[test]
    fn check_all_rucksacks() {
        let rucksacks = vec!["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"]
            .iter()
            .map(|l| Rucksack::from_str(l).unwrap())
            .collect::<Vec<_>>();

        let priority = process_multiple_rucksacks(&rucksacks);
        assert_eq!(priority, Ok(157));
    }
}
