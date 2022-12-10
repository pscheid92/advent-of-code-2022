
use std::str::FromStr;

use anyhow::Context;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, line_ending, space0, u64};
use nom::combinator::{eof, map};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, terminated, tuple};

use aoc2022::*;

fn main() -> anyhow::Result<()> {
    let lines = read(5).context("error reading input")?;
    let packs = pack(lines);

    let stacks = packs[0].join("\n");
    let moves = &packs[1];

    let mut stacks = Stacks::from_str(&stacks)
        .context("error parsing stacks")?;

    let moves = moves
        .iter()
        .map(|l| Move::from_str(l))
        .collect::<Result<Vec<_>, _>>()
        .context("error parsing moves")?;

    stacks.execute_many(&moves);
    println!("result: {}", stacks.get_tops());
    Ok(())
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Move {
    many: usize,
    from: usize,
    to: usize,
}

impl_from_str!(Move);

impl Move {
    fn new(many: usize, from: usize, to: usize) -> Self {
        Move { many, from, to }
    }

    fn parse(s: &str) -> nom::IResult<&str, Self> {
        let parser = (tag("move "), u64, tag(" from "), u64, tag(" to "), u64);
        let parser = terminated(tuple(parser), eof);
        let mut parser = map(parser, |(_, m, _, f, _, t)| Move::new(m as usize, f as usize, t as usize));
        parser(s)
    }
}

#[derive(Debug)]
struct Stacks<> {
    data: Vec<Vec<String>>
}

impl_from_str!(Stacks);

impl Stacks {
    fn new(n: usize) -> Self {
        Stacks { data: vec![Vec::new(); n] }
    }

    fn parse(s: &str) -> nom::IResult<&str, Stacks> {
        let stack_element = alt((tag::<_, &str, _>("   "), delimited(tag("["), alpha1, tag("]"))));
        let stack_lines = many1(terminated(separated_list1(char(' '), stack_element), line_ending));
        let end_line = terminated(many0(delimited(space0, digit1, space0)), eof);
        let parser = terminated(stack_lines, end_line);

        let mut parser = map(parser, |x| {
            let mut stacks = Stacks::new(x[0].len());

            for l in x.iter().rev() {
                for (i, e) in l.iter().enumerate() {
                    if e.trim().is_empty() {
                        continue;
                    }
                    stacks.data[i].push(e.to_string());
                }
            }

            stacks
        });

        parser(s)
    }

    fn execute(&mut self, mov: Move) {
        for _ in 0..mov.many {
            match self.data[mov.from-1].pop() {
                Some(v) => self.data[mov.to-1].push(v),
                None => panic!("this should never happen!"),
            }
        }
    }

    fn execute_many(&mut self, moves: &[Move]) {
        for &m in moves {
            self.execute(m);
        }
    }

    fn get_tops(&self) -> String {
        let mut tops = String::new();

        for i in self.data.iter() {
            if let Some(v) = i.last() {
                tops += v;
            }
        }

        tops
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn getting_top_state_works() -> anyhow::Result<()> {
        let mut stacks = Stacks::new(3);

        stacks.data[0] = vec![];
        stacks.data[1] = vec!["A".to_string(), "B".to_string()];
        stacks.data[2] = vec!["Z".to_string()];

        assert_eq!(stacks.get_tops(), "BZ");

        Ok(())
    }

    #[test]
    fn move_execution_works() -> anyhow::Result<()> {
        let mut stacks = Stacks::new(2);
        stacks.data[0] = vec!["A".to_string(), "B".to_string()];

        stacks.execute(Move::new(2, 1, 2));

        assert_eq!(stacks.data[0], Vec::new() as Vec<String>);
        assert_eq!(stacks.data[1], vec!["B", "A"]);

        Ok(())
    }

    #[test]
    fn move_execution_of_many_works() -> anyhow::Result<()> {
        let mut stacks = Stacks::new(2);
        stacks.data[0] = vec!["A".to_string(), "B".to_string()];

        let moves = vec![
            Move::new(2, 1, 2),
            Move::new(1, 2, 1),
        ];

        stacks.execute_many(&moves);

        assert_eq!(stacks.data[0], vec!["A"]);
        assert_eq!(stacks.data[1], vec!["B"]);

        Ok(())
    }

    #[test]
    fn can_parse_input() -> anyhow::Result<()> {
        let packs = pack(get_test_input());
        let stacks = packs.get(0).context("error getting boards")?;
        let moves = packs.get(1).context("error getting moves")?;

        let stacks = Stacks::from_str(&stacks.join("\n"))
            .context("error parsing stacks")?;

        let moves = moves
            .iter()
            .map(|l| Move::from_str(l))
            .collect::<Result<Vec<_>, _>>()
            .context("error parsing moves")?;

        // check stacks
        assert_eq!(stacks.data.len(), 3);
        assert_eq!(stacks.data[0], vec!["Z"]);
        assert_eq!(stacks.data[1], vec!["M", "N"]);
        assert_eq!(stacks.data[2], vec!["P", "C", "D"]);

        // check moves
        assert_eq!(moves.len(), 4);
        assert_eq!(moves[0], Move::new(1, 2, 1));
        assert_eq!(moves[1], Move::new(3, 1, 3));
        assert_eq!(moves[2], Move::new(2, 2, 1));
        assert_eq!(moves[3], Move::new(1, 1, 2));

        Ok(())
    }

    fn get_test_input() -> Vec<String> {
        vec![
            "        [D]",
            "    [N] [C]",
            "[Z] [M] [P]",
            " 1   2   3",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
            .into_iter()
            .map(|s| String::from(s))
            .collect()
    }
}
