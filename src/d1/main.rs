use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() -> Result<(), io::Error> {
    let v = read_lines("src/d1/input.txt")?
        .map(|v| v.unwrap().parse::<u64>().ok())
        .collect::<Vec<_>>();

    let max: u64 = v
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .sorted()
        .rev()
        .take(3)
        .sum();

    println!("max: {max:?}");
    Ok(())
}
