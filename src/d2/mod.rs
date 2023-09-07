use crate::PuzzleRun;
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]

struct Part1;

struct Part2;

pub fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1), Box::new(Part2)]
}
#[derive(Debug, Clone, Copy)]
enum Choice1 {
    Rock,
    Paper,
    Scissors,
}

impl Choice1 {
    fn score(self, c: Choice2) -> u64 {
        println!("{:?} scoring {:?}", self, c);
        match self {
            Choice1::Rock => match c {
                // rock
                Choice2::Rock => 1 + 3,
                Choice2::Paper => 2 + 6, // paper beats rock
                Choice2::Scissors => 3 + 0,
            },
            Choice1::Paper => match c {
                // paper
                Choice2::Rock => 1 + 0,
                Choice2::Paper => 2 + 3,
                Choice2::Scissors => 3 + 6, // scossors beat paper
            },
            Choice1::Scissors => match c {
                Choice2::Rock => 1 + 6, // rock beats scissors
                Choice2::Paper => 2 + 0,
                Choice2::Scissors => 3 + 3,
            },
        }
    }

    fn loose(&self) -> Choice2 {
        match self {
            Self::Rock => Choice2::Paper,
            Self::Paper => Choice2::Scissors,
            Self::Scissors => Choice2::Rock,
        }
    }

    fn draw(&self) -> Choice2 {
        match self {
            Self::Rock => Choice2::Rock,
            Self::Paper => Choice2::Paper,
            Self::Scissors => Choice2::Scissors,
        }
    }

    fn win(&self) -> Choice2 {
        match self {
            Self::Rock => Choice2::Scissors,
            Self::Paper => Choice2::Rock,
            Self::Scissors => Choice2::Paper,
        }
    }
}

impl TryFrom<char> for Choice1 {
    type Error = &'static str;

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            'A' => Ok(Choice1::Rock),
            'B' => Ok(Choice1::Paper),
            'C' => Ok(Choice1::Scissors),
            _ => Err("Invalid choice"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Choice2 {
    Rock,     // loose
    Paper,    // draw
    Scissors, // win
}

impl Choice2 {
    fn as_directed(&self, other: Choice1) -> Choice2 {
        match self {
            Self::Rock => other.win(),
            Self::Paper => other.draw(),
            Self::Scissors => other.loose(),
        }
    }
}
impl TryFrom<char> for Choice2 {
    type Error = &'static str;

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            'X' => Ok(Choice2::Rock),
            'Y' => Ok(Choice2::Paper),
            'Z' => Ok(Choice2::Scissors),
            _ => Err("Invalid choice"),
        }
    }
}

trait ParseLine {
    fn parse_as_choice(&self) -> (Choice1, Choice2);
}

impl ParseLine for &str {
    fn parse_as_choice(&self) -> (Choice1, Choice2) {
        (
            self.chars().nth(0).unwrap().try_into().unwrap(),
            self.chars().nth(2).unwrap().try_into().unwrap(),
        )
    }
}

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        crate::read_file("src/d2/input.txt")
    }

    fn run(&self, input: &str) -> String {
        let score = input
            .lines()
            .map(|l| l.parse_as_choice())
            .map(|(c1, c2)| (c1, c2, c1.score(c2)))
            //.inspect(|v| println!("v: {:?}", v))
            .map(|(_, _, score)| score)
            .sum::<u64>();

        format!("score: {score}")
    }
}

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        crate::read_file("src/d2/input.txt")
    }

    fn run(&self, input: &str) -> String {
        let score = input
            .lines()
            .map(|l| l.parse_as_choice())
            .inspect(|c| println!("line: {:?}", c))
            .map(|(c1, c2)| (c1, c2, c1.score(c2.as_directed(c1))))
            .inspect(|v| println!("v: {:?}", v))
            .map(|(_, _, score)| score)
            .sum::<u64>();

        format!("score = {score}")
    }
}
