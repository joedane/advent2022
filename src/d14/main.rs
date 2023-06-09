use anyhow::{anyhow, Result};
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }
}

impl Default for Coord {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
struct Line {
    points: Vec<Coord>,
}

impl FromIterator<Coord> for Line {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Coord>,
    {
        let mut points = vec![];
        for i in iter {
            points.push(i);
        }
        Line { points }
    }
}
#[derive(Clone, Copy, Debug)]
enum State {
    Empty,
    Wall,
    Sand,
}

mod topo {

    use crate::{Coord, Line, State};
    use colored::Colorize;
    pub(crate) struct Topo {
        height: usize,
        width: usize,
        x_offset: usize,
        data: Vec<State>,
    }

    pub(crate) enum StepResult {
        Moved(Coord),
        Stopped,
        Off,
    }

    impl std::fmt::Debug for Topo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            colored::control::set_override(true);
            f.write_fmt(format_args!("x_offset: {}\n", self.x_offset));
            for y in 0..self.height {
                f.write_fmt(format_args!("{}\t", y));
                for x in self.x_offset..(self.x_offset + self.width) {
                    let s = match self[Coord::new(x, y)] {
                        State::Empty => ".",
                        State::Sand => "o",
                        State::Wall => "#",
                    };
                    if x == 500 {
                        let ss = s.red().bold();
                        f.write_str(&ss)?;
                    } else {
                        f.write_str(s)?;
                    }
                }
                f.write_str("\n")?;
            }
            Ok(())
        }
    }
    impl Topo {
        fn offset(&self, x: usize, y: usize) -> usize {
            y * self.width + (x - self.x_offset)
        }

        fn make(lines: &Vec<Line>) -> Self {
            let (mut max_x, mut max_y, mut min_x, mut min_y) = (0, 0, usize::MAX, usize::MAX);
            for line in lines.iter() {
                for c in line.points.iter() {
                    if c.x > max_x {
                        max_x = c.x;
                    }
                    if c.x < min_x {
                        min_x = c.x;
                    }
                    if c.y > max_y {
                        max_y = c.y;
                    }
                    if c.y < min_y {
                        min_y = c.y;
                    }
                }
            }
            println!(
                "width: {}, height: {}",
                max_x - min_x + 1,
                max_y - min_y + 1
            );
            let mut data: Vec<State> = vec![State::Empty; (max_x - min_x + 1) * (max_y + 1)];

            Self {
                height: max_y + 1,
                width: max_x - min_x + 1,
                x_offset: min_x,
                data,
            }
        }

        pub(crate) fn new(lines: Vec<Line>) -> Self {
            let mut topo = Self::make(&lines);

            let mut c: Coord = Default::default();

            // mark the first point.   then we can just mark remaining points below
            topo[lines[0].points[0]] = State::Wall;

            for line in lines.iter() {
                for i in 0..line.points.len() - 1 {
                    let start = line.points[i];
                    let end = line.points[i + 1];

                    if start.x == end.x {
                        // vertical
                        let dy: i32 = end.y as i32 - start.y as i32;
                        c.x = start.x;
                        match dy.cmp(&0) {
                            std::cmp::Ordering::Greater => {
                                for j in 1..=(dy as usize) {
                                    c.y = start.y + j;
                                    topo[c] = State::Wall;
                                }
                            }
                            std::cmp::Ordering::Less => {
                                let dy = (-dy) as usize;
                                for j in 1..=dy {
                                    c.y = start.y - j;
                                    topo[c] = State::Wall;
                                }
                            }
                            _ => {}
                        }
                    } else if start.y == end.y {
                        // horizontal
                        c.y = start.y;
                        let dx: i32 = end.x as i32 - start.x as i32;
                        match dx.cmp(&0) {
                            std::cmp::Ordering::Greater => {
                                for j in 1..=(dx as usize) {
                                    c.x = start.x + j;
                                    topo[c] = State::Wall;
                                }
                            }
                            std::cmp::Ordering::Less => {
                                let dx = (-dx) as usize;
                                for j in 1..=dx {
                                    c.x = start.x - j;
                                    topo[c] = State::Wall;
                                }
                            }
                            _ => {}
                        }
                    } else {
                        panic!()
                    }
                }
            }
            topo
        }

        pub(crate) fn drop_grain(&self, p: Coord) -> Option<Coord> {
            let mut c = p;
            loop {
                match self.step(c) {
                    StepResult::Off => return None,
                    StepResult::Stopped => return Some(c),
                    StepResult::Moved(next_step) => {
                        c = next_step;
                    }
                }
            }
        }
        pub(crate) fn step(&self, p: Coord) -> StepResult {
            if p.y + 1 == self.height {
                // fall off the bottom
                return StepResult::Off;
            }
            let mut c: Coord = Default::default();

            c.x = p.x;
            c.y = p.y + 1;
            if let State::Empty = self[c] {
                // move down
                return StepResult::Moved(c);
            }

            // diag left?
            if p.x == self.x_offset {
                return StepResult::Off;
            }
            c.x = p.x - 1;
            c.y = p.y + 1;
            if let State::Empty = self[c] {
                return StepResult::Moved(c);
            }

            // diag right?
            if p.x + 1 == self.width {
                return StepResult::Off;
            }
            c.x = p.x + 1;
            c.y = p.y + 1;
            if let State::Empty = self[c] {
                return StepResult::Moved(c);
            }
            StepResult::Stopped
        }
    }

    impl std::ops::Index<Coord> for Topo {
        type Output = State;

        fn index(&self, index: Coord) -> &Self::Output {
            &self.data[self.offset(index.x, index.y)]
        }
    }

    impl std::ops::IndexMut<Coord> for Topo {
        fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
            let offset = self.offset(index.x, index.y);
            &mut self.data[offset]
        }
    }
}
fn parse_line(input: &str) -> Result<Line> {
    input
        .split("->")
        .map(|s| s.trim())
        .map(|s| match s.find(',') {
            Some(i) => Ok(Coord {
                x: s[0..i].parse().unwrap(),
                y: s[i + 1..].parse().unwrap(),
            }),
            None => Err(anyhow!("bad coordinate: {s}")),
        })
        .collect()
}

fn parse_lines<T>(lines: T) -> Result<Vec<Line>>
where
    T: Iterator<Item = &'static str>,
{
    lines
        .into_iter()
        .map(|s| s.trim())
        .map(|l| parse_line(l))
        .collect::<Result<Vec<Line>, _>>()
}
fn main() -> Result<()> {
    use topo::{StepResult, Topo};

    let input = include_str!("input.txt");

    let mut topo = Topo::new(parse_lines(input.lines())?);

    let c = Coord::new(500, 0);
    let mut i = 0;
    loop {
        println!("step {}\n{:?}", i, topo);
        i += 1;
        match topo.drop_grain(c) {
            None => break,
            Some(to) => {
                topo[to] = State::Sand;
            }
        }
    }
    println!("{} grains", i - 1);
    Ok(())
}

#[cfg(test)]
mod test {

    use super::topo::*;
    use super::*;

    #[test]
    fn test_parse() {
        let line = parse_line("508,146 -> 513,146").unwrap();
        assert_eq!(
            line,
            Line {
                points: vec![Coord::new(508, 146), Coord::new(513, 146)]
            }
        );
    }

    #[test]
    fn test_print() {
        let input = "498,4 -> 498,6 -> 496,6
                503,4 -> 502,4 -> 502,9 -> 494,9";
        let topo = Topo::new(parse_lines(input.lines()).unwrap());
        println!("{topo:?}");
    }

    #[test]
    fn test_step() {
        let input = "498,4 -> 498,6 -> 496,6
                503,4 -> 502,4 -> 502,9 -> 494,9";
        let topo = Topo::new(parse_lines(input.lines()).unwrap());
        let mut c = Coord::new(500, 0);
        assert_eq!(topo.drop_grain(c), Some(Coord::new(500, 8)));
    }

    #[test]
    fn test_steps() {
        let input = "498,4 -> 498,6 -> 496,6
                503,4 -> 502,4 -> 502,9 -> 494,9";
        let mut topo = Topo::new(parse_lines(input.lines()).unwrap());
        let c = Coord::new(500, 0);
        let mut i = 1;
        loop {
            println!("{}\n{:?}", i, topo);
            i += 1;
            match topo.drop_grain(c) {
                None => break,
                Some(to) => {
                    topo[to] = State::Sand;
                }
            }
        }
    }

    #[test]
    fn test_print_large() {
        let input = include_str!("input.txt");
        let topo = Topo::new(parse_lines(input.lines()).unwrap());
        println!("{topo:?}");
    }
}
