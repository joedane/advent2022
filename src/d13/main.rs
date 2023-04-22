use anyhow::Result;
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
                for i in 0..v1.len() {
                    if i >= v2.len() {
                        return Some(Less);
                    }
                    match v1[i].partial_cmp(&v2[i]) {
                        Some(Less) => {
                            return Some(Less);
                        }
                        Some(Greater) => {
                            return Some(Greater);
                        }
                        None => {
                            panic!()
                        }
                        _ => {}
                    }
                }
                if v1.len() < v2.len() {
                    return Some(Less);
                } else {
                    return Some(Equal);
                }
            }
        }
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
fn main() -> Result<()> {
    let input = r"[1,1,3,1,1]
    [1,1,5,1,1]
    
    [[1],[2,3,4]]
    [[1],4]
    
    [9]
    [[8,7,6]]
    
    [[4,4],4,4]
    [[4,4],4,4,4]
    
    [7,7,7,7]
    [7,7,7]
    
    []
    [3]
    
    [[[]]]
    [[]]
    
    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]";

    for (i, mut chunk) in input.lines().chunks(3).into_iter().enumerate() {
        let (src1, src2) = (chunk.next().unwrap().trim(), chunk.next().unwrap().trim());
        let (set1, set2) = (parse_item(src1)?, parse_item(src2)?);
    }

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
    }
}
