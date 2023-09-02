use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub(crate) struct Coord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Coord {
    pub(crate) fn new(x: usize, y: usize) -> Self {
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
    fn new(ul: (usize, usize), br: (usize, usize)) -> Self {
        Self {
            upper_left: Coord::new(ul.0, ul.1),
            lower_right: Coord::new(br.0, br.1),
        }
    }

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
}

impl From<Coord> for Bounds {
    fn from(c: Coord) -> Self {
        Bounds::new_from_coord(c)
    }
}
use colored::Colorize;
pub(crate) struct Topo {
    data: HashMap<Coord, State>,
    bounds: Bounds,
}

pub(crate) enum StepResult {
    Moved(Coord),
    Stopped,
    Off,
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
        let mut bounds = Bounds::new_from_coord(Coord::new(0, 0));
        for c in iter {
            data.insert(c, State::Wall);
            bounds.update(c);
        }
        Topo { data, bounds }
    }
}

impl std::fmt::Debug for Topo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        /*
        colored::control::set_override(true);
        let Bounds {
            upper_left: ul,
            lower_right: lr,
        } = self.maybe_expand(Bounds::new((0, 0), (20, 20)));

        for y in 0..(lr.y + 1) {
            for x in ul.x..(lr.x + 1) {
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
        */
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

    /*
    pub(crate) fn maybe_expand(&self, b: Bounds) -> Bounds {
        let sb = self.bounds;
        self.bounds = Bounds {
            upper_left: Coord::new(
                std::cmp::min(sb.upper_left.x, b.upper_left.x),
                std::cmp::max(sb.upper_left.y, b.upper_left.y),
            ),
            lower_right: Coord::new(
                std::cmp::max(sb.lower_right.x, b.lower_right.x),
                std::cmp::min(sb.lower_right.y, b.lower_right.y),
            ),
        };
        self.bounds
    }
    */

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
        Topo { data, bounds }
    }

    pub(crate) fn coord_iter(&self) -> impl Iterator<Item = (Coord, State)> + '_ {
        self.data
            .iter()
            .map(|(c_ref, s_ref)| (c_ref.to_owned(), s_ref.to_owned()))
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

    /**
     * to where will the grain at Coord p next move?
     * This does not update the model.  Panic there's not a graid
     * at coord p.
     */
    pub(crate) fn step(&self, p: Coord) -> StepResult {
        if !matches!(self[p], State::Sand) {
            panic!("no grain currently at Coord {:?}", p);
        }
        let current_bound = self.bounds;

        if p.y >= current_bound.lower_right.y {
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
