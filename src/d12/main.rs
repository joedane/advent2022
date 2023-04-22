use anyhow::Result;
use itertools::Itertools;
use ndarray::{Array2, Ix};
use priority_queue::PriorityQueue;
use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn up(&self) -> Pos {
        Pos::new(self.x, self.y + 1)
    }

    fn down(&self) -> Pos {
        Pos::new(self.x, self.y - 1)
    }

    fn left(&self) -> Pos {
        Pos::new(self.x - 1, self.y)
    }

    fn right(&self) -> Pos {
        Pos::new(self.x + 1, self.y)
    }

    fn as_index(&self) -> (Ix, Ix) {
        (self.y, self.x)
    }
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("P[{}, {}]", self.x, self.y))
    }
}

type Map = Array2<char>;

fn neighbors<M>(map: M, n: Pos) -> Vec<Pos>
where
    M: Borrow<Map>,
{
    let mut v = vec![];
    let map = map.borrow();
    let (rows, cols) = map.dim();

    // if n == self.start {
    //     if n.x > 0 {
    //         v.push(n.left());
    //     }
    //     if n.x < cols - 1 {
    //         v.push(n.right());
    //     }
    //     if n.y > 0 {
    //         v.push(n.down());
    //     }
    //     if n.y < rows - 1 {
    //         v.push(n.up());
    //     }
    //     return v;
    // }
    let (x, y) = (n.x, n.y);
    if x > 0 {
        let pos = Pos::new(x - 1, y);
        if map[pos.as_index()] <= (map[[y, x]] as u8 + 1) as char {
            v.push(Pos::new(x - 1, y));
        }
    }
    if x < cols - 1 {
        let pos = Pos::new(x + 1, y);
        if map[pos.as_index()] <= (map[[y, x]] as u8 + 1) as char {
            v.push(Pos::new(x + 1, y));
        }
    }
    if y > 0 {
        let pos = Pos::new(x, y - 1);
        if map[pos.as_index()] <= (map[[y, x]] as u8 + 1) as char {
            v.push(Pos::new(x, y - 1));
        }
    }
    if y < rows - 1 {
        let pos = Pos::new(x, y + 1);
        if map[pos.as_index()] <= (map[[y, x]] as u8 + 1) as char {
            v.push(Pos::new(x, y + 1));
        }
    }
    v
}

fn reconstruct_path(cameFrom: &HashMap<Pos, Pos>, mut current: Pos) -> Vec<Pos> {
    let mut v = vec![current];

    while cameFrom.contains_key(&current) {
        current = *cameFrom.get(&current).unwrap();
        v.push(current);
    }
    v.into_iter().rev().collect()
}

fn locate(map: &Array2<char>, item: char) -> Option<Pos> {
    let (rows, cols) = map.dim();
    for row in 0..rows {
        for col in 0..cols {
            if map[(row, col)] == item {
                return Some(Pos::new(col, row));
            }
        }
    }
    None
}
fn locate_all(map: &Array2<char>, item: char) -> Vec<Pos> {
    let (rows, cols) = map.dim();
    let mut v = vec![];
    for row in 0..rows {
        for col in 0..cols {
            if map[(row, col)] == item {
                v.push(Pos::new(col, row));
            }
        }
    }
    v
}

fn astar<M: Borrow<Map>>(start: Pos, goal: Pos, map: M) -> Option<Vec<Pos>> {
    let map = map.borrow();
    let h =
        |n: Pos| ((n.x as i64 - goal.x as i64).abs() + (n.y as i64 - goal.y as i64).abs()) as u64;
    let mut open_set: PriorityQueue<Pos, std::cmp::Reverse<u64>> = PriorityQueue::new();
    open_set.push(start, std::cmp::Reverse(h(start)));
    let mut came_from: HashMap<Pos, Pos> = HashMap::new();
    let mut g_score: HashMap<Pos, u64> = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score: HashMap<Pos, u64> = HashMap::new();
    f_score.insert(start, h(start));

    while open_set.len() > 0 {
        //        println!("{} items in the open set:", open_set.len());
        // for (i, (p, _)) in open_set.iter().enumerate() {
        //     print!(
        //         "{}. f{}/g{}\t",
        //         i + 1,
        //         f_score.get(p).copied().unwrap_or(0),
        //         g_score.get(p).copied().unwrap_or(0)
        //     );
        //     for p in reconstruct_path(&came_from, *p) {
        //         print!(" {:?} ", p);
        //     }
        //     print!("\n");
        // }
        let (current, _) = open_set.pop().unwrap();
        //        println!("current: {:?}", current);
        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }
        let tentative_g = g_score
            .get(&current)
            .map(|v| *v)
            .map_or(u64::MAX, |v| v + 1);
        for neighbor in neighbors(map, current).into_iter() {
            let n = neighbor;
            if tentative_g < g_score.get(&neighbor).map(|v| *v).unwrap_or(u64::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g);
                let this_f = tentative_g + h(neighbor);
                // println!(
                //     "from {:?} adding neighbor {:?} with g_score {} and f_score {}",
                //     current, neighbor, tentative_g, this_f
                // );
                f_score.insert(neighbor, this_f);
                open_set.push(neighbor, std::cmp::Reverse(this_f));
            }
        }
    }
    None
}

fn to_u8(c: char) -> u8 {
    c as u8
}

fn parse_map<'a, T>(lines: T) -> Result<Array2<char>>
where
    T: Iterator<Item = &'a str>,
{
    let lines: Vec<&str> = lines.map(|s| s.trim()).collect();
    let (nrows, ncols) = (lines.len(), lines[0].len());

    let data: Vec<char> = lines.into_iter().flat_map(|s| s.chars()).collect();

    Array2::from_shape_vec((nrows, ncols), data).map_err(anyhow::Error::from)
}

fn part1(m: Array2<char>, start: Pos, end: Pos) {
    match astar(start, end, m) {
        Some(path) => {
            println!("path has length: {}", path.len() - 1);
            for (i, p) in path.iter().enumerate() {
                println!("\t{}: {:?}", i, p);
            }
        }
        None => {
            println!("no path found");
        }
    }
}

fn part2<M: Borrow<Map>>(m: M, end: Pos) -> Option<Vec<Pos>> {
    let a_pos = locate_all(m.borrow(), 'a');
    let mut best_score = usize::MAX;
    let mut best_path: Option<Vec<Pos>> = None;

    for &start in a_pos.iter() {
        if let Some(path) = astar(start, end, m.borrow()) {
            println!(
                "path starting at {:?} has length: {}",
                start,
                path.len() - 1
            );
            if path.len() < best_score {
                best_score = path.len();
                best_path.replace(path);
            }
            // for (i, p) in path.iter().enumerate() {
            //     println!("\t{}: {:?}", i, p);
            // }
        }
    }
    best_path
}

fn main1() -> Result<()> {
    let data = include_str!("input.txt");
    let mut char_map: HashMap<char, u32> = HashMap::new();
    for line in data.lines() {
        for c in line.chars() {
            char_map.entry(c).and_modify(|i| *i += 1).or_insert(1);
        }
    }
    let keys_sorted: Vec<_> = char_map.keys().sorted().collect();
    for k in keys_sorted {
        println!("{}: {}", k, char_map.get(k).unwrap());
    }
    Ok(())
}
fn main() -> Result<()> {
    // let data = r"Sabqponm
    // abcryxxl
    // accszExk
    // acctuvwj
    // abdefghi";
    let data = include_str!("input.txt");
    let mut m = parse_map(data.lines())?;
    let (start, end) = (locate(&m, 'S').unwrap(), locate(&m, 'E').unwrap());
    m[start.as_index()] = 'a';
    m[end.as_index()] = 'z';
    //    part1(m);
    match part2(m, end) {
        Some(path) => {
            println!("best path had length {}", path.len() - 1);
        }
        None => {
            println!("no paths found");
        }
    }
    Ok(())
}
