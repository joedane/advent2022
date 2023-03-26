use anyhow::{anyhow, Result};
use regex::Regex;
use std::str::FromStr;

struct M {
    items: Vec<u32>,
    op: Box<dyn Fn(u32) -> u32>,
    divisor: u32,
    throw_on_true: usize,
    throw_on_false: usize,
}

impl M {
    fn new(divisor: u32, throw_on_false: usize) -> Self {
        Self {
            items: vec![],
            op: Box::new(|foo| foo),
            divisor,
            throw_on_true: 0,
            throw_on_false,
        }
    }
}
fn load<T: Iterator<Item = &'static str>>(input: T) -> Result<Vec<M>> {
    let mut vecs = vec![];
    let mut items: Option<Vec<u32>> = None;
    let mut divisor: Option<u32> = None;

    for line in input {
        if let Some(caps) = Regex::new(r"Monkey (\d+):")?.captures(line) {
            // current_index = usize::from_str(foo.as_str())?;
            let id = usize::from_str(
                caps.get(1)
                    .ok_or(anyhow!("failed to match monkey ID"))?
                    .as_str(),
            )?;
        } else if let Some(caps) = Regex::new("Starting items: (.+)$")?.captures(line) {
            let item_str = caps
                .get(1)
                .ok_or(anyhow!("failed to match items"))?
                .as_str();
            let item_vec: Result<Vec<u32>> = item_str
                .split(',')
                .map(|s| u32::from_str(s.trim()))
                .map(|r| r.map_err())
                .collect();

            println!("len: {}", item_vec.len());
        //                .ok_or(anyhow!("failed to read items"));
        } else if let Some(caps) = Regex::new(r"Test: divisible by (\d+)")?.captures(line) {
            divisor = Some(u32::from_str(
                caps.get(1)
                    .ok_or(anyhow!("failed to match divisor"))?
                    .as_str(),
            )?);
        } else if let Some(caps) = Regex::new(r"If false: throw to monkey (\d+)")?.captures(line) {
            let tif = Some(usize::from_str(
                caps.get(1)
                    .ok_or(anyhow!("failed to match false"))?
                    .as_str(),
            )?);
            vecs.push(M::new(divisor, tif));
        }
    }
    Ok(vecs)
}
fn main() -> Result<()> {
    let data = include_str!("input.txt");
    let ms = load(data.lines())?;
    for (i, m) in ms.iter().enumerate() {
        println!("Monkey {} has divisor {}", i, m.divisor)
    }
    Ok(())
}
