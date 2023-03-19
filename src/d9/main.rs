use anyhow::{anyhow, Context, Result};
use std::collections::HashSet;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
enum Dir {
    U,
    D,
    R,
    L,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
struct Pos {
    x: i64,
    y: i64,
}

fn follow(mut from: Pos, next: Pos) -> Pos {
    let x_diff = next.x - from.x;
    let y_diff = next.y - from.y;
    if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
        return from;
    }
    if from.y == next.y {
        assert!(x_diff.abs() == 2);
        if x_diff > 0 {
            from.x += 1;
        } else {
            from.x -= 1;
        }
    } else if from.x == next.x {
        assert!(y_diff.abs() == 2);
        if y_diff > 0 {
            from.y += 1;
        } else {
            from.y -= 1;
        }
    } else {
        if next.y > from.y {
            from.y += 1;
        } else {
            from.y -= 1;
        }
        if next.x > from.x {
            from.x += 1;
        } else {
            from.x -= 1;
        }
    }
    from
}

fn _part1<T>(lines: T) -> Result<usize>
where
    T: Iterator<Item = &'static str>,
{
    let (mut head, mut tail) = (Pos { x: 0, y: 0 }, Pos { x: 0, y: 0 });
    let mut seen: HashSet<Pos> = Default::default();
    seen.insert(tail);

    for inst in lines {
        //println!("instr: {inst}");
        let (dir_str, n_str) = inst
            .trim()
            .split_once(' ')
            .ok_or(anyhow!("invalid line: {inst}"))?;
        let dir = Dir::from_str(dir_str).context(format!("Invalid direction: {}", dir_str))?;
        let n = u32::from_str(n_str)?;
        for _ in 0..n {
            match dir {
                Dir::U => head.y += 1,
                Dir::D => head.y -= 1,
                Dir::R => head.x += 1,
                Dir::L => head.x -= 1,
            }
            tail = follow(tail, head);
            //println!("head: {:?}\ttail: {:?}", head, tail);
            seen.insert(tail);
        }
    }
    Ok(seen.len())
}
fn part2<T>(lines: T) -> Result<usize>
where
    T: Iterator<Item = &'static str>,
{
    let mut knots: [Pos; 10] = Default::default();
    let mut seen: HashSet<Pos> = Default::default();
    seen.insert(knots[9]);

    for inst in lines {
        let (dir_str, n_str) = inst
            .trim()
            .split_once(' ')
            .ok_or(anyhow!("invalid line: {inst}"))?;
        let dir = Dir::from_str(dir_str)?;
        let n = u32::from_str(n_str)?;
        println!("instr: {inst}");
        for _ in 0..n {
            match dir {
                Dir::U => knots[0].y += 1,
                Dir::D => knots[0].y -= 1,
                Dir::R => knots[0].x += 1,
                Dir::L => knots[0].x -= 1,
            }
            for i in 1..knots.len() {
                knots[i] = follow(knots[i], knots[i - 1]);
            }
            dump(&knots);
            seen.insert(knots[9]);
        }
    }
    Ok(seen.len())
}

fn dump(knots: &[Pos; 10]) {
    for row in (0..6).rev() {
        for col in 0..6 {
            print!(
                "{} ",
                match (0..10).find(|p| knots[*p as usize].y == row && knots[*p as usize].x == col) {
                    Some(p) => p.to_string(),
                    None => ".".to_string(),
                }
            );
        }
        println!("");
    }
    println!("\n\n");
}
fn main() -> Result<()> {
    let data = include_str!("input.txt");
    //println!("part 1: {}", part1(data.lines()).unwrap());
    println!("part 2: {}", part2(data.lines()).unwrap());
    Ok(())
}

mod test {

    #[test]
    fn test_part2() {
        let data = r"R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";

        println!("part 2: {}", crate::part2(data.lines()).unwrap());
    }
}
