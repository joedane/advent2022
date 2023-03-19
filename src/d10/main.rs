use anyhow::{anyhow, Result};
use std::str::FromStr;

enum Instr {
    Noop,
    Add(i32),
}

impl Instr {
    fn decode(line: &str) -> Result<Self> {
        if line.starts_with("noop") {
            Ok(Instr::Noop)
        } else if line.starts_with("addx") {
            let (_, val) = line
                .split_once(' ')
                .ok_or(anyhow!("invalid instruction: {}", line))?;
            let addend = i32::from_str(val)?;
            Ok(Instr::Add(addend))
        } else {
            Err(anyhow!("Invalid instruction: {line}"))
        }
    }
}

fn part1<T: Iterator<Item = &'static str>>(data: T) -> Result<i32> {
    let mut cycle_count: usize = 0;
    let mut register_vals: [i32; 250] = [0; 250];
    let mut register: i32 = 1;

    for line in data.map(|s| s.trim()) {
        if cycle_count > 220 {
            break;
        }
        match Instr::decode(line).unwrap() {
            // remove this?
            Instr::Noop => {
                register_vals[cycle_count] = register;
                cycle_count += 1;
            }
            Instr::Add(val) => {
                register_vals[cycle_count] = register;
                cycle_count += 1;
                register_vals[cycle_count] = register;
                cycle_count += 1;
                register += val;
            }
        }
    }

    Ok(20 * register_vals[20]
        + 60 * register_vals[60]
        + 100 * register_vals[100]
        + 140 * register_vals[140]
        + 180 * register_vals[180]
        + 220 * register_vals[220])
}
fn main() -> Result<()> {
    let data = include_str!("test-input.txt");
    println!("Part 1: {}", part1(data.lines()).unwrap());
    Ok(())
}

mod test {

    #[test]
    fn test_part1() {
        let data = include_str!("test-input.txt");
        println!("Part 1: {}", crate::part1(data.lines()).unwrap());
    }
}
