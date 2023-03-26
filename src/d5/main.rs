#![feature(let_else)]

use anyhow::{Context, Result};
use regex::Regex;
use std::str::FromStr;

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for (n, stack) in stacks.iter().enumerate() {
        println!("stack {}: {:?}", n + 1, stack);
    }
}

fn runit<'a, I, F>(lines: I, stacks: &mut Vec<Vec<char>>, part_fn: F) -> Result<()>
where
    I: Iterator<Item = &'a str>,
    F: Fn(u8, &mut Vec<char>, &mut Vec<char>) -> Result<()>,
{
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in lines {
        print_stacks(stacks);
        println!("instr: {line}");
        let Some(captures) = re.captures(line) else { continue };
        let (count, from, to) = (
            u8::from_str(captures.get(1).unwrap().as_str()).unwrap(),
            u8::from_str(captures.get(2).unwrap().as_str()).unwrap(),
            u8::from_str(captures.get(3).unwrap().as_str()).unwrap(),
        );

        assert!(from != to);
        assert!(from > 0 && (from as usize) <= stacks.len());
        assert!(to > 0 && (to as usize) <= stacks.len());

        unsafe {
            let from_stack = &mut *(stacks.get_unchecked_mut(from as usize - 1) as *mut _);
            let to_stack = &mut *(stacks.get_unchecked_mut(to as usize - 1) as *mut _);
            part_fn(count, from_stack, to_stack)?;
        }
    }
    Ok(())
}

fn _part1(count: u8, from_stack: &mut Vec<char>, to_stack: &mut Vec<char>) -> Result<()> {
    if count > 0 {
        let obj = from_stack.pop().context("stack {from} is empty")?;
        to_stack.push(obj);
        _part1(count - 1, from_stack, to_stack)?;
    }
    Ok(())
}

fn part2(count: u8, from_stack: &mut Vec<char>, to_stack: &mut Vec<char>) -> Result<()> {
    if count > 0 {
        let start = from_stack.len() - count as usize;
        let items = from_stack.drain(start..);
        to_stack.extend(items);
    }
    Ok(())
}

fn main() -> Result<()> {
    let data = include_str!("input.txt");
    let mut lines = data.lines();
    let mut stacks: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    for _ in 0..8 {
        let line = lines.next().unwrap();
        for i in 0..9 {
            let c = line.chars().nth(1 + i * 4).unwrap();
            if c.is_ascii_alphabetic() {
                stacks[i].push(c);
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }
    let _ = lines.next();
    let _ = lines.next();

    runit(lines, &mut stacks, part2)?;

    println!("FINAL ...");
    print_stacks(&stacks);
    for (n, stack) in stacks.iter().enumerate() {
        println!("stack {n}: {}", stack[stack.len() - 1]);
    }
    Ok(())
}
