use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub(crate) struct Coord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Coord {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct Line {
    pub(crate) points: Vec<Coord>,
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
pub(crate) enum State {
    Empty,
    Wall,
    Sand,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Bounds {
    pub upper_left: Coord,
    pub lower_right: Coord,
}

impl Bounds {
    fn new_from_coord(c: Coord) -> Self {
        Self {
            upper_left: c,
            lower_right: c,
        }
    }

    fn get_width(&self) -> usize {
        self.lower_right.x - self.upper_left.x
    }

    fn get_height(&self) -> usize {
        self.lower_right.y - self.upper_left.y
    }

    fn update(&mut self, c: Coord) {
        self.upper_left = Coord::new(
            std::cmp::min(self.upper_left.x, c.x),
            std::cmp::min(self.upper_left.y, c.y),
        );
        self.lower_right = Coord::new(
            std::cmp::max(self.lower_right.x, c.x),
            std::cmp::max(self.lower_right.y, c.y),
        );
    }

    fn maybe_expand(&self, ul: Coord, lr: Coord) -> Self {
        Self {
            upper_left: Coord::new(
                std::cmp::min(self.upper_left.x, ul.x),
                std::cmp::max(self.upper_left.y, ul.y),
            ),
            lower_right: Coord::new(
                std::cmp::max(self.lower_right.x, lr.x),
                std::cmp::min(self.lower_right.y, lr.y),
            ),
        }
    }
}

impl From<Coord> for Bounds {
    fn from(c: Coord) -> Self {
        Bounds::new_from_coord(c)
    }
}
pub(crate) struct Topo {
    data: HashMap<Coord, State>,
    bounds: Bounds,
    active: Vec<Coord>,
    floor: Option<usize>,
}

pub(crate) enum StepResult {
    Moved(Coord, Coord),
    Stopped(Coord),
    Off(Coord),
}

/*
pub(crate) struct TopoCoordIterator {
    x_offset: usize,
    row: usize,
    col: usize,
    height: usize,
    width: usize,
}

impl Iterator for TopoCoordIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.height {
            if self.col < self.width {
                let c = Coord::new(self.col + self.x_offset, self.row);
                if self.col + 1 < self.width {
                    self.col += 1;
                } else {
                    self.row += 1;
                    self.col = 0;
                }
                return Some(c);
            }
        }
        None
    }
}
*/

impl FromIterator<Coord> for Topo {
    fn from_iter<I: IntoIterator<Item = Coord>>(iter: I) -> Self {
        let mut data: HashMap<Coord, State> = Default::default();
        let mut a_coord: Option<Coord> = None;

        for c in iter {
            if a_coord.is_none() {
                a_coord.replace(c);
            }
            data.insert(c, State::Wall);
        }
        if a_coord.is_none() {
            panic!();
        }
        let a_coord = a_coord.unwrap();
        let mut bounds = Bounds::new_from_coord(a_coord);
        for c in data.keys() {
            bounds.update(*c);
        }
        Topo {
            data,
            bounds,
            active: vec![],
            floor: None,
        }
    }
}

impl std::fmt::Debug for Topo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;
        colored::control::set_override(true);

        let Bounds {
            upper_left: ul,
            lower_right: lr,
        } = self.bounds;

        let display_floor = match self.floor {
            Some(floor) => floor,
            None => lr.y,
        };
        let (start_x, end_x) = match self.floor {
            Some(_) => (ul.x - 2, lr.x + 2),
            None => (ul.x, lr.x),
        };

        //.maybe_expand(Coord::new(0, 0), Coord::new(20, 20));

        for y in 0..(display_floor + 1) {
            for x in start_x..(end_x + 1) {
                let s = match self[Coord::new(x, y)] {
                    _ if y == lr.y + 2 => "=",
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
    /*
        fn offset(&self, x: usize, y: usize) -> usize {
            y * self.width + (x - self.x_offset)
        }
    */

    pub(crate) fn get_bounds(&self) -> Bounds {
        self.bounds
    }

    pub(crate) fn get_width(&self) -> usize {
        self.bounds.get_width()
    }

    pub(crate) fn get_height(&self) -> usize {
        self.bounds.get_height()
    }

    pub(crate) fn get_x_offset(&self) -> usize {
        self.bounds.upper_left.x
    }

    pub(crate) fn with_floor(&mut self) {
        self.floor = Some(self.bounds.lower_right.y + 2);
    }

    pub(crate) fn from_lines(lines: Vec<Line>) -> Self {
        let mut data: HashMap<Coord, State> = Default::default();
        let mut c: Coord = Default::default();
        let mut bounds = Bounds::new_from_coord(lines[0].points[0]);

        // mark the first point.   then we can just mark remaining points below
        data.insert(lines[0].points[0], State::Wall);

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
                            for j in 0..=(dy as usize) {
                                c.y = start.y + j;
                                data.insert(c, State::Wall);
                                bounds.update(c);
                            }
                        }
                        std::cmp::Ordering::Less => {
                            let dy = (-dy) as usize;
                            for j in 0..=dy {
                                c.y = start.y - j;
                                data.insert(c, State::Wall);
                                bounds.update(c);
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
                            for j in 0..=(dx as usize) {
                                c.x = start.x + j;
                                data.insert(c, State::Wall);
                                bounds.update(c);
                            }
                        }
                        std::cmp::Ordering::Less => {
                            let dx = (-dx) as usize;
                            for j in 0..=dx {
                                c.x = start.x - j;
                                data.insert(c, State::Wall);
                                bounds.update(c);
                            }
                        }
                        _ => {}
                    }
                } else {
                    panic!()
                }
            }
        }
        Topo {
            data,
            bounds,
            active: vec![],
            floor: None,
        }
    }

    pub(crate) fn coord_iter(&self) -> impl Iterator<Item = (Coord, State)> + '_ {
        self.data
            .iter()
            .map(|(c_ref, s_ref)| (c_ref.to_owned(), s_ref.to_owned()))
    }

    pub(crate) fn drop_at(&mut self, c: Coord) -> bool {
        if matches!(self[c], State::Empty) {
            self[c] = State::Sand;
            self.active.push(c);
            true
        } else {
            false
        }
    }

    /**
     * update the model to reflect one time step.
     * might be nice if we didn't allocate here.
     */
    pub fn step(&mut self) -> Vec<StepResult> {
        let mut rs = vec![];
        let actives = std::mem::take(&mut self.active);

        //println!("{:?}", self);

        for c in actives.into_iter() {
            let result = self.next_pos(c);
            if let StepResult::Moved(from, to) = result {
                self[from] = State::Empty;
                self[to] = State::Sand;
                self.active.push(to);
            }
            rs.push(result);
        }
        rs
    }

    fn next_pos(&self, p: Coord) -> StepResult {
        let current_bound = self.bounds;

        if let Some(floor) = self.floor {
            if p.y + 1 == floor {
                return StepResult::Stopped(p);
            }
        } else {
            if p.y == current_bound.lower_right.y {
                return StepResult::Off(p);
            }
        }

        let mut c: Coord = Default::default();

        c.x = p.x;
        c.y = p.y + 1;
        if let State::Empty = self[c] {
            // move down
            return StepResult::Moved(p, c);
        }

        // diag left?
        c.x = p.x - 1;
        c.y = p.y + 1;
        if let State::Empty = self[c] {
            if self.floor.is_some() || c.x >= current_bound.upper_left.x {
                return StepResult::Moved(p, c);
            } else {
                return StepResult::Off(p);
            }
        }

        // diag right?
        c.x = p.x + 1;
        c.y = p.y + 1;
        if let State::Empty = self[c] {
            if self.floor.is_some() || c.x <= current_bound.lower_right.x {
                return StepResult::Moved(p, c);
            } else {
                return StepResult::Off(p);
            }
        }
        StepResult::Stopped(p)
    }

    /*
        fn step_coord(&mut self, p: Coord) -> StepResult {
            if !matches!(self[p], State::Sand) {
                panic!("no grain currently at Coord {:?}", p);
            }
            let current_bound = self.bounds;

            if p.y >= current_bound.lower_right.y {
                if self.floor {
                    if p.y == current_bound.lower_right.y {
                        return StepResult::Moved(Coord::new(p.x, p.y + 1));
                    } else {
                        return StepResult::Stopped;
                    }
                } else {
                    // fall off the bottom
                    return StepResult::Off;
                }
            }
            let mut c: Coord = Default::default();

            c.x = p.x;
            c.y = p.y + 1;
            if let State::Empty = self[c] {
                // move down
                return StepResult::Moved(c);
            }

            // diag left?
            if p.x <= current_bound.upper_left.x {
                return StepResult::Off;
            }
            c.x = p.x - 1;
            c.y = p.y + 1;
            if let State::Empty = self[c] {
                return StepResult::Moved(c);
            }

            // diag right?
            if p.x + 1 >= current_bound.lower_right.x {
                return StepResult::Off;
            }
            c.x = p.x + 1;
            c.y = p.y + 1;
            if let State::Empty = self[c] {
                return StepResult::Moved(c);
            }
            StepResult::Stopped
        }
    */
}
impl std::ops::Index<Coord> for Topo {
    type Output = State;

    fn index(&self, index: Coord) -> &Self::Output {
        self.data.get(&index).unwrap_or(&State::Empty)
    }
}
impl std::ops::IndexMut<Coord> for Topo {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let r = self.data.entry(index).or_insert(State::Empty);
        self.bounds.update(index);
        r
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

pub(crate) fn parse_lines<'a, T>(lines: T) -> Result<Vec<Line>>
where
    T: Iterator<Item = &'a str>,
{
    lines
        .into_iter()
        .map(|s| s.trim())
        .map(|l| parse_line(l))
        .collect::<Result<Vec<Line>, _>>()
}

#[cfg(test)]
mod test {

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
        let topo = Topo::from_lines(parse_lines(input.lines()).unwrap());
        println!("{topo:?}");
    }

    #[test]
    fn test_print_large() {
        let input = include_str!("input.txt");
        let topo = Topo::from_lines(parse_lines(input.lines()).unwrap());
        println!("{topo:?}");
    }
}
