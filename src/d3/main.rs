use itertools::Itertools;

fn find_duplicate(line: &str) -> Option<char> {
    if line.len() % 2 != 0 {
        panic!("invalid str length ({}) for input {}", line.len(), line);
    } else {
        let (l, r) = line.split_at(line.len() / 2);
        l.chars().filter(|c| r.contains(*c)).take(1).next()
    }
}

fn part2<'a, L>(mut lines: L) -> Result<u64, &'static str>
where
    L: Iterator<Item = &'a str>,
{
    let sum: u64 = std::iter::from_fn(move || lines.next_tuple::<(&'a str, &'a str, &'a str)>())
        .map(|(a, b, c)| {
            println!("a: {a}\nb: {b}\nc: {c}");
            let found = a
                .chars()
                .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
                .unwrap();
            println!("found: {found}");
            <u8 as Into<u64>>::into(priority(found as u8))
        })
        .sum::<u64>();
    Ok(sum)
}

fn part1(lines: std::str::Lines) -> u64 {
    lines
        .map(|l| {
            let dup = find_duplicate(l).unwrap();
            let p: u64 = priority(dup as u8).into();
            p
        })
        .sum::<u64>()
}

fn priority(in_val: u8) -> u8 {
    if in_val >= 97 && in_val <= 122 {
        (in_val - 96).into()
    } else if in_val >= 65 && in_val <= 90 {
        (in_val - 64 + 26).into()
    } else {
        panic!("invalid character: {in_val}");
    }
}
fn main() {
    let data = include_str!("input.txt");
    println!("part 1: {}", part1(data.lines()));
    println!("part 2: {}", part2(data.lines()).unwrap());
}

#[cfg(test)]
mod test {
    use crate::find_duplicate;
    use crate::priority;

    #[test]
    fn test_find_dup() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let c = find_duplicate(s);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), 'p');
        assert_eq!(priority(b'p'), 16);

        let s = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let c = find_duplicate(s);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), 'L');
        assert_eq!(priority(b'L'), 38);

        let s = "PmmdzqPrVvPwwTWBwg";
        let c = find_duplicate(s);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), 'P');
        assert_eq!(priority(b'P'), 42);

        let s = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let c = find_duplicate(s);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), 'v');
        assert_eq!(priority(b'v'), 22);

        let s = "ttgJtRGJQctTZtZT";
        let c = find_duplicate(s);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), 't');
        assert_eq!(priority(b't'), 20);

        let s = "CrZsJsPPZsGzwwsLwLmpwMDw";
        let c = find_duplicate(s);
        assert!(c.is_some());
        assert_eq!(c.unwrap(), 's');
        assert_eq!(priority(b's'), 19);
    }

    #[test]
    fn test_part2() {
        let lines = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];

        assert_eq!(crate::part2(lines.into_iter()).unwrap(), 18);
    }
}
