use clap::{Parser, Subcommand};
use std::path::Path;

mod d1;
mod d14;
mod d15;
mod d2;
mod d3;
mod d4;
mod d5;

pub trait PuzzleRun {
    fn input_data(&self) -> anyhow::Result<&str>;

    fn run(&self, input: &str) -> String;
}

fn read_file<P: AsRef<Path>>(filename: P) -> anyhow::Result<&'static str> {
    let data = std::fs::read_to_string(filename)?;
    Ok(data.leak())
}

#[derive(Parser)]
struct Args {
    /// Enable debug logging
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Which day's puzzles to run?
    #[command(subcommand)]
    day: Option<PuzzleDay>,
}
#[derive(Subcommand, Debug)]
enum PuzzleDay {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day14,
    Day15,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let puzzles = match args.day {
        Some(PuzzleDay::Day1) => d1::get_runs(),
        Some(PuzzleDay::Day2) => d2::get_runs(),
        Some(PuzzleDay::Day3) => d3::get_runs(),
        Some(PuzzleDay::Day4) => d4::get_runs(),
        Some(PuzzleDay::Day5) => d5::get_runs(),
        Some(PuzzleDay::Day14) => d14::get_runs(),
        Some(PuzzleDay::Day15) => d15::get_runs(),
        _ => {
            println!("not found: {:?}", args.day);
            panic!()
        }
    };

    for puzzle in puzzles {
        println!("{}", puzzle.run(puzzle.input_data()?));
    }
    Ok(())
}
