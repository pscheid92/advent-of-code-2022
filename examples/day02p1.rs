extern crate aoc2022;

use aoc2022::*;
use std::str::FromStr;

use RoundResult::{Draw, Lost, Won};
use Shape::{Paper, Rock, Scissors};

fn main() {
    let lines = read(2).unwrap();
    let game = Game::try_from(lines).unwrap();
    println!("score: {}", game.score());
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
}

impl FromStr for Shape {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err("error"),
        }
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
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        let opponent = match tokens.next() {
            Some(v) => v,
            None => return Err(""),
        };

        let me = match tokens.next() {
            Some(v) => v,
            None => return Err(""),
        };

        let opponent = Shape::from_str(opponent)?;
        let me = Shape::from_str(me)?;

        Ok(Round::new(me, opponent))
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
    type Error = &'static str;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let rounds = value
            .iter()
            .map(|l| Round::from_str(l))
            .collect::<Result<_, _>>()?;

        Ok(Game { rounds })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use RoundResult::{Draw, Lost, Won};
    use Shape::{Paper, Rock, Scissors};

    #[test]
    fn read_shape_from_string() {
        assert_eq!(Shape::from_str("A"), Ok(Rock));
        assert_eq!(Shape::from_str("X"), Ok(Rock));
        assert_eq!(Shape::from_str("B"), Ok(Paper));
        assert_eq!(Shape::from_str("Y"), Ok(Paper));
        assert_eq!(Shape::from_str("C"), Ok(Scissors));
        assert_eq!(Shape::from_str("Z"), Ok(Scissors));
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
