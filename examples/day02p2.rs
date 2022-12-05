extern crate aoc2022;

use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use nom::IResult;
use nom::character::complete::{char, one_of};
use nom::combinator::{eof, map, map_res};
use nom::sequence::{separated_pair, terminated};

use aoc2022::*;
use RoundResult::{Draw, Lost, Won};
use Shape::{Paper, Rock, Scissors};

fn main() -> Result<()> {
    let lines = read(2).context("error reading input")?;
    let game = Game::try_from(lines).context("error parsing game")?;
    println!("score: {}", game.score());
    Ok(())
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn parse(s: &str) -> IResult<&str, Shape> {
        let mut parser = map_res(terminated(one_of("ABCXYZ"), eof), |c: char| match c {
            'A' => Ok(Rock),
            'B' => Ok(Paper),
            'C' => Ok(Scissors),
            _ => Err(anyhow!("error")),
        });

        parser(s)
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
enum RoundResult {
    Lost,
    Draw,
    Won,
}

impl RoundResult {
    fn score(&self) -> u32 {
        match self {
            Lost => 0,
            Draw => 3,
            Won => 6,
        }
    }

    fn parse(s: &str) -> IResult<&str, RoundResult> {
        let mut parser = map_res(terminated(one_of("ABCXYZ"), eof), |c: char| match c {
            'X' => Ok(Lost),
            'Y' => Ok(Draw),
            'Z' => Ok(Won),
            _ => Err(anyhow!("error")),
        });

        parser(s)
    }
}

struct Round {
    opponent: Shape,
    result: RoundResult,
}

impl Round {
    fn new(opponent: Shape, result: RoundResult) -> Self {
        Round { opponent, result }
    }

    fn derive_my_move(&self) -> Shape {
        match (self.opponent, self.result) {
            (Rock, Lost) => Scissors,
            (Rock, Draw) => Rock,
            (Rock, Won) => Paper,

            (Paper, Lost) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Won) => Scissors,

            (Scissors, Lost) => Paper,
            (Scissors, Draw) => Scissors,
            (Scissors, Won) => Rock,
        }
    }

    fn score(&self) -> u32 {
        let me = self.derive_my_move();
        me.score() + self.result.score()
    }

    fn parse(s: &str) -> IResult<&str, Round> {
        let parser = separated_pair(Shape::parse, char(' '), RoundResult::parse);
        let mut parser = map(parser, |(o, r)| Round::new(o, r));
        parser(s)
    }
}

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn score(&self) -> u32 {
        self.rounds.iter().map(|r| r.score()).sum()
    }
}

impl TryFrom<Vec<String>> for Game {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let rounds = value
            .iter()
            .map(|l| Round::from_str(l))
            .collect::<Result<_, _>>()?;

        Ok(Game { rounds })
    }
}

impl_from_str!(Shape);
impl_from_str!(RoundResult);
impl_from_str!(Round);

#[cfg(test)]
mod tests {
    use RoundResult::{Draw, Lost, Won};
    use Shape::{Paper, Rock, Scissors};

    use super::*;

    #[test]
    fn read_shape_from_string() {
        assert_eq!(Shape::from_str("A").unwrap(), Rock);
        assert_eq!(Shape::from_str("B").unwrap(), Paper);
        assert_eq!(Shape::from_str("C").unwrap(), Scissors);
        assert!(Shape::from_str("").is_err());
        assert!(Shape::from_str("a").is_err());
        assert!(Shape::from_str("G").is_err());
        assert!(Shape::from_str("AX").is_err());
    }

    #[test]
    fn read_result_from_string() {
        assert_eq!(RoundResult::from_str("X").unwrap(), Lost);
        assert_eq!(RoundResult::from_str("Y").unwrap(), Draw);
        assert_eq!(RoundResult::from_str("Z").unwrap(), Won);
        assert!(Shape::from_str("").is_err());
        assert!(Shape::from_str("x").is_err());
        assert!(Shape::from_str("G").is_err());
        assert!(Shape::from_str("XA").is_err());
    }

    #[test]
    fn check_derivation_of_move() {
        assert_eq!(Round::new(Rock, Lost).derive_my_move(), Scissors);
        assert_eq!(Round::new(Rock, Draw).derive_my_move(), Rock);
        assert_eq!(Round::new(Rock, Won).derive_my_move(), Paper);

        assert_eq!(Round::new(Paper, Lost).derive_my_move(), Rock);
        assert_eq!(Round::new(Paper, Draw).derive_my_move(), Paper);
        assert_eq!(Round::new(Paper, Won).derive_my_move(), Scissors);

        assert_eq!(Round::new(Scissors, Lost).derive_my_move(), Paper);
        assert_eq!(Round::new(Scissors, Draw).derive_my_move(), Scissors);
        assert_eq!(Round::new(Scissors, Won).derive_my_move(), Rock);
    }

    #[test]
    fn score_on_round_result() {
        assert_eq!(Lost.score(), 0);
        assert_eq!(Draw.score(), 3);
        assert_eq!(Won.score(), 6);
    }

    #[test]
    fn score_a_round() {
        assert_eq!(Round::new(Rock, Draw).score(), 4);
        assert_eq!(Round::new(Paper, Lost).score(), 1);
        assert_eq!(Round::new(Scissors, Won).score(), 7);
    }

    #[test]
    fn i_can_score_a_game() {
        let rounds = vec![
            Round::new(Rock, Draw),
            Round::new(Paper, Lost),
            Round::new(Scissors, Won),
        ];
        let game = Game { rounds };
        assert_eq!(game.score(), 12);
    }
}
