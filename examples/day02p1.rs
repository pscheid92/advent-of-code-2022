extern crate aoc2022;
extern crate core;

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
            'A' | 'X' => Ok(Rock),
            'B' | 'Y' => Ok(Paper),
            'C' | 'Z' => Ok(Scissors),
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
}

struct Round {
    me: Shape,
    opponent: Shape,
}

impl Round {
    fn new(me: Shape, opponent: Shape) -> Self {
        Round { me, opponent }
    }

    fn play(&self) -> RoundResult {
        match (self.me, self.opponent) {
            (Rock, Scissors) => Won,
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lost,

            (Paper, Rock) => Won,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lost,

            (Scissors, Paper) => Won,
            (Scissors, Scissors) => Draw,
            (Scissors, Rock) => Lost,
        }
    }

    fn score(&self) -> u32 {
        let result = self.play();
        self.me.score() + result.score()
    }

    fn parse(s: &str) -> IResult<&str, Round> {
        let parser = separated_pair(Shape::parse, char(' '), Shape::parse);
        let mut parser = map(parser, |(o, m)| Round::new(m, o));
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
impl_from_str!(Round);

#[cfg(test)]
mod tests {
    use RoundResult::{Draw, Lost, Won};
    use Shape::{Paper, Rock, Scissors};

    use super::*;

    #[test]
    fn read_shape_from_string() {
        assert_eq!(Shape::from_str("A").unwrap(), Rock);
        assert_eq!(Shape::from_str("X").unwrap(), Rock);
        assert_eq!(Shape::from_str("B").unwrap(), Paper);
        assert_eq!(Shape::from_str("Y").unwrap(), Paper);
        assert_eq!(Shape::from_str("C").unwrap(), Scissors);
        assert_eq!(Shape::from_str("Z").unwrap(), Scissors);
        assert!(Shape::from_str("").is_err());
        assert!(Shape::from_str("a").is_err());
        assert!(Shape::from_str("G").is_err());
        assert!(Shape::from_str("AX").is_err());
    }

    #[test]
    fn decide_if_i_have_won() {
        use RoundResult::*;
        use Shape::*;

        assert_eq!(Round::new(Rock, Rock).play(), Draw);
        assert_eq!(Round::new(Paper, Paper).play(), Draw);
        assert_eq!(Round::new(Scissors, Scissors).play(), Draw);

        assert_eq!(Round::new(Rock, Scissors).play(), Won);
        assert_eq!(Round::new(Rock, Paper).play(), Lost);

        assert_eq!(Round::new(Paper, Rock).play(), Won);
        assert_eq!(Round::new(Paper, Scissors).play(), Lost);

        assert_eq!(Round::new(Scissors, Paper).play(), Won);
        assert_eq!(Round::new(Scissors, Rock).play(), Lost);
    }

    #[test]
    fn score_on_round_result() {
        assert_eq!(Lost.score(), 0);
        assert_eq!(Draw.score(), 3);
        assert_eq!(Won.score(), 6);
    }

    #[test]
    fn score_a_round() {
        assert_eq!(Round::new(Paper, Rock).score(), 8);
        assert_eq!(Round::new(Rock, Paper).score(), 1);
        assert_eq!(Round::new(Scissors, Scissors).score(), 6);
    }

    #[test]
    fn i_can_score_a_game() {
        let rounds = vec![
            Round::new(Paper, Rock),
            Round::new(Rock, Paper),
            Round::new(Scissors, Scissors),
        ];
        let game = Game { rounds };
        assert_eq!(game.score(), 15);
    }
}
