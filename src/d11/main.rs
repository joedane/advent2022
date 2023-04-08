use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use regex::Regex;
use std::{num::ParseIntError, str::FromStr};

struct M {
    items: Vec<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    divisor: u64,
    throw_on_true: usize,
    throw_on_false: usize,
    num_inspections: u64,
}

impl M {
    fn new(
        items: Vec<u64>,
        op: Box<dyn Fn(u64) -> u64>,
        divisor: u64,
        throw_on_true: usize,
        throw_on_false: usize,
    ) -> Self {
        Self {
            items,
            op,
            divisor,
            throw_on_true,
            throw_on_false,
            num_inspections: 0,
        }
    }
}

fn make_monkey<'a, I>(input: I) -> Result<M>
where
    I: Iterator<Item = &'a str>,
{
    let mut items: Option<Vec<u64>> = None;
    let mut divisor: Option<u64> = None;
    let mut tit: Option<usize> = None;
    let mut tif: Option<usize> = None;
    let mut op: Option<Box<dyn Fn(u64) -> u64>> = None;

    for line in input {
        if let Some(caps) = Regex::new(r"Monkey (\d+):")?.captures(line) {
            // current_index = usize::from_str(foo.as_str())?;
            let id = usize::from_str(
                caps.get(1)
                    .ok_or(anyhow!("failed to match monkey ID"))?
                    .as_str(),
            )?;
            println!("read monkey {}", id);
        } else if let Some(caps) = Regex::new("Starting items: (.+)$")?.captures(line) {
            let item_str = caps
                .get(1)
                .ok_or(anyhow!("failed to match items"))?
                .as_str();
            items = Some(
                item_str
                    .split(',')
                    .map(|s| u64::from_str(s.trim()))
                    .collect::<Result<Vec<u64>, ParseIntError>>()?,
            );
        } else if let Some(caps) = Regex::new(r"Test: divisible by (\d+)")?.captures(line) {
            divisor = Some(u64::from_str(
                caps.get(1)
                    .ok_or(anyhow!("failed to match divisor"))?
                    .as_str(),
            )?);
        } else if let Some(caps) =
            Regex::new(r"Operation: new = old ([+*]) (\d+|old)")?.captures(line)
        {
            let operand_str = caps
                .get(2)
                .ok_or(anyhow!("failed to read operation"))?
                .as_str();

            let opcode = caps.get(1).ok_or(anyhow!("failed to read op"))?.as_str();

            if operand_str == "old" {
                op = Some(match opcode {
                    "+" => Box::new(|x| x + x),
                    "*" => Box::new(|x| x * x),
                    _ => bail!("dsfafds"),
                });
            } else {
                let operand = u64::from_str(operand_str)?;
                op = Some(match opcode {
                    "+" => Box::new(move |x| x + operand),
                    "*" => Box::new(move |x| x * operand),
                    _ => bail!("FOOBAR"),
                });
            }
        } else if let Some(caps) = Regex::new(r"If true: throw to monkey (\d+)")?.captures(line) {
            tit = Some(usize::from_str(
                caps.get(1).ok_or(anyhow!("failed to match true"))?.as_str(),
            )?);
        } else if let Some(caps) = Regex::new(r"If false: throw to monkey (\d+)")?.captures(line) {
            tif = Some(usize::from_str(
                caps.get(1)
                    .ok_or(anyhow!("failed to match false"))?
                    .as_str(),
            )?);
        }
    }
    Ok(M::new(
        items.expect("items"),
        op.expect("op"),
        divisor.expect("divisor"),
        tit.expect("tit"),
        tif.expect("tif"),
    ))
}
fn load<T: Iterator<Item = &'static str>>(input: T) -> Result<Vec<M>> {
    let mut monkeys = vec![];

    for chunk in input.chunks(7).into_iter() {
        monkeys.push(make_monkey(chunk)?);
    }

    Ok(monkeys)
}

fn part1(mut ms: Vec<M>) {
    for i in 0..20 {
        println!("starting round {}", i);
        for i in 0..ms.len() {
            println!("looking at monkey {}", i);
            let items = std::mem::take(&mut ms[i].items);
            for item in items {
                ms[i].num_inspections += 1;
                let level = (ms[i].op)(item);
                let level = level / 3;
                if level % ms[i].divisor == 0 {
                    let idx = ms[i].throw_on_true;
                    ms[idx].items.push(level);
                } else {
                    let idx = ms[i].throw_on_false;
                    ms[idx].items.push(level);
                }
            }
            show_ms(&ms);
        }
    }
    ms.sort_unstable_by(|a, b| b.num_inspections.partial_cmp(&a.num_inspections).unwrap());

    for i in 0..ms.len() {
        println!("monkey {} inspected {} items", i, ms[i].num_inspections);
    }
    println!(
        "monkey business: {}",
        ms[0].num_inspections * ms[1].num_inspections
    );
}

fn part2(mut ms: Vec<M>) {
    let divisor = ms
        .iter()
        .map(|m| m.divisor)
        .reduce(|acc, e| acc * e)
        .unwrap();
    for i in 0..10000 {
        //println!("starting round {}", i);
        for i in 0..ms.len() {
            //println!("looking at monkey {}", i);
            let items = std::mem::take(&mut ms[i].items);
            for item in items {
                ms[i].num_inspections += 1;
                let level = (ms[i].op)(item);
                let level = level % divisor;
                if level % ms[i].divisor == 0 {
                    let idx = ms[i].throw_on_true;
                    ms[idx].items.push(level);
                } else {
                    let idx = ms[i].throw_on_false;
                    ms[idx].items.push(level);
                }
            }
            //show_ms(&ms);
        }
        for i in 0..ms.len() {
            //println!("monkey {} inspected {} items", i, ms[i].num_inspections);
        }
    }
    ms.sort_unstable_by(|a, b| b.num_inspections.partial_cmp(&a.num_inspections).unwrap());

    for i in 0..ms.len() {
        println!("monkey {} inspected {} items", i, ms[i].num_inspections);
    }
    println!(
        "monkey business: {}",
        ms[0].num_inspections * ms[1].num_inspections
    );
}

fn main() -> Result<()> {
    let data = include_str!("input.txt");
    let ms = load(data.lines())?;
    part2(ms);
    Ok(())
}

fn show_ms(ms: &Vec<M>) {
    for (i, m) in ms.iter().enumerate() {
        println!("Monkey {} has {:?}", i, m.items)
    }
}
