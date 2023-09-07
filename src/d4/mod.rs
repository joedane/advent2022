use crate::PuzzleRun;
use std::str::FromStr;

pub fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

fn split(line: &str) -> Result<(&str, &str), &'static str> {
    line.split_once(',').ok_or("bad line: {line}")
}

fn to_range(desc: &str) -> Result<(u8, u8), &'static str> {
    let (a_str, b_str) = desc.split_once('-').ok_or("bad desc: {desc}")?;
    let a = u8::from_str(a_str).map_err(|_| "bad str: {a_str}")?;
    let b = u8::from_str(b_str).map_err(|_| "bad str: {b_str}")?;
    Ok((a, b))
}
fn included_in(a: (u8, u8), b: (u8, u8)) -> bool {
    (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1)
}

fn overlaps(a: (u8, u8), b: (u8, u8)) -> bool {
    (a.0 <= b.1 && a.1 >= b.0) || (b.0 <= a.1 && b.1 >= a.0)
}

fn check<'a, L, F>(lines: L, predicate: F) -> u64
where
    L: Iterator<Item = &'a str>,
    F: Fn((u8, u8), (u8, u8)) -> bool,
{
    let c = lines
        .map(|l| split(l).unwrap())
        .map(|(a, b)| (to_range(a).unwrap(), to_range(b).unwrap()))
        .filter(|(range_a, range_b)| predicate(*range_a, *range_b))
        .count();
    c as u64
}

fn _part1_old<'a, L>(lines: L) -> Result<u64, &'static str>
where
    L: Iterator<Item = &'a str>,
{
    let c = lines
        .map(|l| split(l).unwrap())
        .map(|(a, b)| (to_range(a).unwrap(), to_range(b).unwrap()))
        .filter(|(range_a, range_b)| included_in(*range_a, *range_b))
        .count();
    Ok(c as u64)
}

struct Part1;

impl crate::PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        crate::read_file("src/d4/input.txt")
    }

    fn run(&self, input: &str) -> String {
        format!("{}", Part1::_run(input))
    }
}

impl Part1 {
    fn _run(input: &str) -> u64 {
        check(input.lines(), included_in)
    }
}

impl crate::PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        crate::read_file("src/d4/input.txt")
    }

    fn run(&self, input: &str) -> String {
        format!("{}", Part2::_run(input))
    }
}

struct Part2;

impl Part2 {
    fn _run(input: &str) -> u64 {
        check(input.lines(), overlaps)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_included() {
        assert!(included_in((2, 8), (3, 7)));
        assert!(included_in((3, 7), (2, 8)));

        assert!(!included_in((5, 7), (7, 9)));
    }

    #[test]
    fn test_overlaps() {
        assert!(overlaps((2, 8), (3, 7)));
        assert!(overlaps((3, 7), (2, 8)));
        assert!(overlaps((5, 7), (7, 9)));
        assert!(overlaps((7, 9), (5, 7)));

        assert!(!overlaps((2, 3), (4, 5)));
    }
}
