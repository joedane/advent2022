use anyhow::Result;
use ndarray::{Array2, Ix};
use priority_queue::PriorityQueue;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

struct Map {
    map: Array2<char>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn neighbors(&self, n: Pos) -> Vec<Pos> {
        let mut v = vec![];
        let map = &self.map;
        let (rows, cols) = map.dim();

        if n == self.end {
            panic!("this should not happen");
        }
        if n == self.start {
            if n.x > 0 {
                v.push(n.left());
            }
            if n.x < cols - 1 {
                v.push(n.right());
            }
            if n.y > 0 {
                v.push(n.down());
            }
            if n.y < rows - 1 {
                v.push(n.up());
            }
            return v;
        }
        let (x, y) = (n.x, n.y);
        if x > 0 {
            let pos = Pos::new(x - 1, y);
            if map[pos.as_index()] <= (map[[y, x]] as u8 + 1) as char || pos == self.end {
                v.push(Pos::new(x - 1, y));
            }
        }
        if x < cols - 1 {
            let pos = Pos::new(x + 1, y);
            if map[pos.as_index()] <= (map[[y, x]] as u8 + 1) as char || pos == self.end {
                v.push(Pos::new(x + 1, y));
            }
        }
        if y > 0 {
            let pos = Pos::new(x, y - 1);
            if map[pos.as_index()] <= (map[[y, x]] as u8 + 1) as char || pos == self.end {
                v.push(Pos::new(x, y - 1));
            }
        }
        if y < rows - 1 {
            let pos = Pos::new(x, y + 1);
            if map[pos.as_index()] <= (map[[y, x]] as u8 + 1) as char || pos == self.end {
                v.push(Pos::new(x, y + 1));
            }
        }
        v
    }
}
fn reconstruct_path(cameFrom: HashMap<Pos, Pos>, mut current: Pos) -> Vec<Pos> {
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
fn astar(start: Pos, goal: Pos, map: Map) -> Option<Vec<Pos>> {
    let h =
        |n: Pos| ((n.x as i64 - goal.x as i64).abs() + (n.y as i64 - goal.y as i64).abs()) as u64;
    let mut open_set: PriorityQueue<Pos, u64> = PriorityQueue::new();
    open_set.push(start, h(start));
    let mut came_from: HashMap<Pos, Pos> = HashMap::new();
    let mut g_score: HashMap<Pos, u64> = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score: HashMap<Pos, u64> = HashMap::new();
    f_score.insert(start, h(start));

    while open_set.len() > 0 {
        println!("open set has {} items", open_set.len());
        let (current, _) = open_set.pop().unwrap();
        if current == goal {
            return Some(reconstruct_path(came_from, current));
        }
        let tentative_g = g_score
            .get(&current)
            .map(|v| *v)
            .map_or(u64::MAX, |v| v + 1);
        for neighbor in map.neighbors(current).into_iter() {
            let n = neighbor;
            println!("N: {:?}", n);
            if tentative_g < g_score.get(&neighbor).map(|v| *v).unwrap_or(u64::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g);
                let this_f = tentative_g + h(neighbor);
                f_score.insert(neighbor, this_f);
                open_set.push(neighbor, this_f);
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
fn main() -> Result<()> {
    let data = r"Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";
    let mut m = parse_map(data.lines())?;
    println!("{:?}", m);
    let (start, end) = (locate(&m, 'S').unwrap(), locate(&m, 'E').unwrap());
    println!("start: {:?}, end: {:?}", start, end);
    m[start.as_index()] = 'a';
    m[end.as_index()] = 'z';

    match astar(start, end, Map { map: m, start, end }) {
        Some(path) => {
            println!("path has length: {}", path.len());
            for p in path.iter() {
                println!("\t{:?}", p);
            }
        }
        None => {
            println!("no path found");
        }
    }
    Ok(())
}
