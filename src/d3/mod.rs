use crate::PuzzleRun;
use itertools::Itertools;

pub fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

fn find_duplicate(line: &str) -> Option<char> {
    if line.len() % 2 != 0 {
        panic!("invalid str length ({}) for input {}", line.len(), line);
    } else {
        let (l, r) = line.split_at(line.len() / 2);
        l.chars().filter(|c| r.contains(*c)).take(1).next()
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        crate::read_file("src/d3/input.txt")
    }

    fn run(&self, input: &str) -> String {
        let mut lines = input.lines();
        let sum: u64 = std::iter::from_fn(move || lines.next_tuple::<(&str, &str, &str)>())
            .map(|(a, b, c)| {
                println!("a: {a}\nb: {b}\nc: {c}");
                let found = a
                    .chars()
                    .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
                    .unwrap();
                println!("found: {found}");
                <u8 as Into<u64>>::into(priority(found as u8))
            })
            .sum::<u64>();

        format!("{}", sum)
    }
}

struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        crate::read_file("src/d3/input.txt")
    }

    fn run(&self, input: &str) -> String {
        format!("{}", self._run(input))
    }
}

impl Part1 {
    fn _run(&self, input: &str) -> u64 {
        input
            .lines()
            .map(|l| {
                let dup = find_duplicate(l).unwrap();
                let p: u64 = priority(dup as u8).into();
                p
            })
            .sum::<u64>()
    }
}

fn priority(in_val: u8) -> u8 {
    if in_val >= 97 && in_val <= 122 {
        (in_val - 96).into()
    } else if in_val >= 65 && in_val <= 90 {
        (in_val - 64 + 26).into()
    } else {
        panic!("invalid character: {in_val}");
    }
}

#[cfg(test)]
mod test {

    use super::{find_duplicate, priority};

    #[test]
    fn test_find_dup() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let c = find_duplicate(s);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), 'p');
        assert_eq!(priority(b'p'), 16);

        let s = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let c = find_duplicate(s);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), 'L');
        assert_eq!(priority(b'L'), 38);

        let s = "PmmdzqPrVvPwwTWBwg";
        let c = find_duplicate(s);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), 'P');
        assert_eq!(priority(b'P'), 42);

        let s = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let c = find_duplicate(s);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), 'v');
        assert_eq!(priority(b'v'), 22);

        let s = "ttgJtRGJQctTZtZT";
        let c = find_duplicate(s);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), 't');
        assert_eq!(priority(b't'), 20);

        let s = "CrZsJsPPZsGzwwsLwLmpwMDw";
        let c = find_duplicate(s);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), 's');
        assert_eq!(priority(b's'), 19);
    }

    #[test]
    fn test_part2() {
        let lines = "vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg";

        assert_eq!(super::Part1::_run(&super::Part1, lines), 18);
    }
}
