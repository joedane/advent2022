use anyhow::{anyhow, Result};
use std::io::{stdout, Write};
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

fn run<T: Iterator<Item = &'static str>>(data: T) -> [i32; 240] {
    let mut cycle_count: usize = 0;
    let mut register_vals: [i32; 240] = [0; 240];
    let mut register: i32 = 1;

    for line in data.map(|s| s.trim()) {
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
    register_vals
}

fn part1<T: Iterator<Item = &'static str>>(data: T) -> Result<i32> {
    let register_vals = run(data);

    Ok(20 * register_vals[20 - 1]
        + 60 * register_vals[60 - 1]
        + 100 * register_vals[100 - 1]
        + 140 * register_vals[140 - 1]
        + 180 * register_vals[180 - 1]
        + 220 * register_vals[220 - 1])
}

fn part2<T: Iterator<Item = &'static str>>(data: T) {
    let register_vals = run(data);
    let mut lock = stdout().lock();
    let mut cycle = 0;

    for val in register_vals.iter() {
        lock.write_all(if (cycle - val).abs() <= 1 {
            &[b'#']
        } else {
            &[b'.']
        })
        .unwrap();
        if (cycle + 1) % 40 == 0 {
            lock.write_all(&[b'\n']).unwrap();
            cycle = 0;
        } else {
            cycle += 1;
        }
    }
}
fn main() -> Result<()> {
    //    let data = include_str!("test-input.txt");
    let data = include_str!("input.txt");
    println!("Part 1: {}", part1(data.lines()).unwrap());
    part2(data.lines());
    Ok(())
}

mod test {

    #[test]
    fn test_part1() {
        let data = include_str!("test-input.txt");
        println!("Part 1: {}", crate::part1(data.lines()).unwrap());
    }
}
