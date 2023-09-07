use crate::PuzzleRun;
use itertools::Itertools;

pub struct D1;

pub fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(D1)]
}
impl PuzzleRun for D1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        crate::read_file("src/d1/input.txt")
    }

    fn run(&self, input: &str) -> String {
        let v = input
            .lines()
            .map(|v| v.parse::<u64>().ok())
            .collect::<Vec<_>>();

        let max: u64 = v
            .split(|line| line.is_none())
            .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
            .sorted()
            .rev()
            .take(3)
            .sum();

        format!("max: {max:?}")
    }
}
