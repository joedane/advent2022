use std::str::FromStr;

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

fn check<'a, L, F>(lines: L, predicate: F) -> Result<u64, &'static str>
where
    L: Iterator<Item = &'a str>,
    F: Fn((u8, u8), (u8, u8)) -> bool,
{
    let c = lines
        .map(|l| split(l).unwrap())
        .map(|(a, b)| (to_range(a).unwrap(), to_range(b).unwrap()))
        .filter(|(range_a, range_b)| predicate(*range_a, *range_b))
        .count();
    Ok(c as u64)
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

fn part1<'a, L>(lines: L) -> Result<u64, &'static str>
where
    L: Iterator<Item = &'a str>,
{
    check(lines, included_in)
}

fn part2<'a, L>(lines: L) -> Result<u64, &'static str>
where
    L: Iterator<Item = &'a str>,
{
    check(lines, overlaps)
}

fn main() {
    let data = include_str!("input.txt");
    println!("part1: {}", part1(data.lines()).unwrap());
    println!("part2: {}", part2(data.lines()).unwrap());
}

#[cfg(test)]
mod test {
    use crate::*;

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
