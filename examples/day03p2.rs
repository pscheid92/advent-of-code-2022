extern crate aoc2022;
extern crate core;

use std::collections::HashSet;
use aoc2022::*;

fn main() {
    let rucksacks = read(3)
        .unwrap()
        .iter()
        .map(|l| Rucksack::new(l))
        .collect::<Vec<_>>();

    let result: u32 = rucksacks
        .chunks_exact(3)
        .map(|x| x.try_into().unwrap())
        .map(|x| process_group(x))
        .sum();

    println!("solution: {}", result);
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

fn get_priority(c: char) -> Result<u32, &'static str> {
    match c {
        'a'..='z' => Ok((c as u32) - ('a' as u32) + 1),
        'A'..='Z' => Ok((c as u32) - ('A' as u32) + 27),
        _ => Err(""),
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

fn process_group(group: &[Rucksack; 3]) -> u32 {
    let badge = identify_badge(group).unwrap();
    let priority = get_priority(badge).unwrap();
    priority
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(process_group(&group1), 18);
        assert_eq!(process_group(&group2), 52);
    }
}
