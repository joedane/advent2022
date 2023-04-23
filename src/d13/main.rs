use anyhow::{anyhow, Result};
use core::fmt::Display;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{char, digit1};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::Finish;
use nom::IResult;

#[derive(Debug, Clone)]
enum Item {
    Int(i32),
    List(Vec<Item>),
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Int(i) => write!(f, "{}", i),
            Item::List(v) => {
                write!(f, "[");
                for i in v.iter() {
                    i.fmt(f);
                    write!(f, ",");
                }
                write!(f, "]");
                Ok(())
            }
        }
    }
}
impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        use Item::*;
        match (self, other) {
            (Int(i), Int(j)) => i.partial_cmp(j),
            (Int(i), List(v)) => {
                let v1 = List(vec![Int(*i)]);
                v1.partial_cmp(&List(v.to_vec()))
            }
            (List(v), Int(i)) => {
                let v2 = List(vec![Int(*i)]);
                self.partial_cmp(&v2)
            }
            (List(v1), List(v2)) => {
                if v1.len() == 0 {
                    if v2.len() > 0 {
                        return Some(Less);
                    } else {
                        return Some(Equal);
                    }
                }
                if v2.len() == 0 {
                    return Some(Greater);
                }
                match v1[0].partial_cmp(&v2[0]) {
                    Some(Less) => Some(Less),
                    Some(Greater) => Some(Greater),
                    _ => List(v1[1..].to_vec()).partial_cmp(&List(v2[1..].to_vec())),
                }
            }
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn digits(input: &str) -> IResult<&str, Item> {
    map(digit1, |d| Item::Int(str::parse(d).unwrap()))(input)
}

fn list(input: &str) -> IResult<&str, Item> {
    map(
        delimited(
            char('['),
            separated_list0(is_a(", "), parse_item),
            char(']'),
        ),
        |o| Item::List(o),
    )(input)
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    let input = input.trim();
    //    println!("parsing: {}", input);
    alt((digits, list))(input)
}

fn part1<T>(input: T) -> Result<()>
where
    T: Iterator<Item = &'static str>,
{
    let mut sum = 0;
    for (i, mut chunk) in input.chunks(3).into_iter().enumerate() {
        let (src1, src2) = (chunk.next().unwrap().trim(), chunk.next().unwrap().trim());
        let (set1, set2) = (parse_item(src1)?, parse_item(src2)?);
        if let Some(std::cmp::Ordering::Less) = set1.partial_cmp(&set2) {
            sum += i + 1;
        }
    }
    println!("sum: {sum}");
    Ok(())
}
fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let mut items: Vec<Item> = input
        .lines()
        .filter_map(|s| {
            let st = s.trim();
            (!st.is_empty()).then_some(st)
        })
        .map(|s| parse_item(s).map(|v| v.1))
        .collect::<Result<Vec<Item>, _>>()?;

    let m1 = parse_item("[[2]]")?.1;
    let m2 = parse_item("[[6]]")?.1;

    items.push(m1.clone());
    items.push(m2.clone());

    items.sort();

    let p1 = items.iter().position(|i| m1.eq(i)).unwrap();
    let p2 = items.iter().position(|i| m2.eq(i)).unwrap();

    for (i, p) in items.iter().enumerate() {
        println!("{i}: {p}");
    }
    println!(
        "p1: {}, p2: {}, result: {}",
        p1 + 1,
        p2 + 1,
        (p1 + 1) * (p2 + 1)
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ord() {
        use std::cmp::Ordering;

        fn do_test_ord(input: &str) -> Option<Ordering> {
            let mut lines = input.lines();
            let ((_, s1), (_, s2)) = (
                parse_item(lines.next().unwrap()).finish().unwrap(),
                parse_item(lines.next().unwrap()).finish().unwrap(),
            );
            s1.partial_cmp(&s2)
        }

        let input = r"[1,1,3,1,1]
        [1,1,5,1,1]";

        assert_eq!(do_test_ord(input), Some(Ordering::Less));

        let input = "[[1],[2,3,4]]
        [[1],4]";
        assert_eq!(do_test_ord(input), Some(Ordering::Less));

        let input = "[9]
        [[8,7,6]]";
        assert_eq!(do_test_ord(input), Some(Ordering::Greater));

        let input = "[[4,4],4,4]
        [[4,4],4,4,4]
    ";
        assert_eq!(do_test_ord(input), Some(Ordering::Less));

        let input = "[7,7,7,7]
        [7,7,7]";
        assert_eq!(do_test_ord(input), Some(Ordering::Greater));

        let input = "[]
        [3]";
        assert_eq!(do_test_ord(input), Some(Ordering::Less));

        let input = "[[[]]]
        [[]]";
        assert_eq!(do_test_ord(input), Some(Ordering::Greater));

        let input = "[1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(do_test_ord(input), Some(Ordering::Greater));
    }
}
