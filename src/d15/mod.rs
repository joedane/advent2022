use crate::PuzzleRun;
use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

use std::collections::{HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Interval {
    low: i64,
    high: i64,
}

impl From<[i64; 2]> for Interval {
    fn from(value: [i64; 2]) -> Self {
        Self {
            low: value[0],
            high: value[1],
        }
    }
}
impl Interval {
    pub(crate) fn new(low: i64, high: i64) -> Self {
        if high < low {
            panic!();
        }
        Self { low, high }
    }
}
struct Intervals {
    intervals: Vec<Interval>,
}

impl From<Vec<Interval>> for Intervals {
    fn from(intervals: Vec<Interval>) -> Self {
        Self { intervals }
    }
}

struct IntervalIter<'a> {
    pos: usize,
    iter: std::slice::Iter<'a, Interval>,
}

impl<'a> Iterator for IntervalIter<'a> {
    type Item = &'a Interval;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl core::fmt::Debug for Intervals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Intervals ({}): {}",
            self.intervals.len(),
            self.intervals
                .iter()
                .map(|i| format!("[{}, {}]", i.low, i.high))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
impl Intervals {
    fn new() -> Self {
        Self {
            intervals: Default::default(),
        }
    }

    fn iter(&self) -> std::slice::Iter<'_, Interval> {
        self.intervals.iter()
    }

    fn interval_count(&self) -> usize {
        self.intervals.len()
    }

    fn compress(&mut self) {
        let mut i = 0;

        while i < self.intervals.len() - 1 {
            if self.intervals[i].high + 1 == self.intervals[i + 1].low {
                self.intervals[i].high = self.intervals[i + 1].high;
                self.intervals.remove(i + 1);
            } else {
                i = i + 1;
            }
        }
    }

    fn merge(&mut self, new_i: Interval) {
        if self.intervals.len() == 0 {
            self.intervals.insert(0, new_i);
            return;
        }

        for i in (0..self.intervals.len()).rev() {
            if self.intervals[i].low <= new_i.low {
                // i is the index of the interval starting immediately before the new interval, if any
                if self.intervals[i].high >= new_i.low {
                    // new interval overlaps with lowest existing interval
                    if new_i.high > self.intervals[i].high {
                        self.intervals[i].high = new_i.high;
                        let j = i + 1;
                        while j < self.intervals.len() && self.intervals[j].low <= new_i.high {
                            if self.intervals[j].high > new_i.high {
                                self.intervals[i].high = self.intervals[j].high;
                            }
                            self.intervals.remove(j);
                        }
                    }
                } else {
                    // new interval does not overlap with lowest existing interval
                    self.intervals.insert(i + 1, new_i);
                    let j = i + 2;
                    while j < self.intervals.len() && self.intervals[j].low <= new_i.high {
                        if self.intervals[j].high > new_i.high {
                            self.intervals[i + 1].high = self.intervals[j].high;
                        }
                        self.intervals.remove(j);
                    }
                }
                return;
            }
        }
        // if we make it here, we're inserting a new lowest interval
        self.intervals.insert(0, new_i);
        while 1 < self.intervals.len() && self.intervals[1].low <= new_i.high {
            if self.intervals[1].high > new_i.high {
                self.intervals[0].high = self.intervals[1].high;
            }
            self.intervals.remove(1);
        }
    }

    fn span(&self) -> u32 {
        self.intervals
            .iter()
            .map(|i| ((i.high - i.low) + 1) as u32)
            .reduce(|acc, i| acc + i)
            .unwrap_or(0u32)
    }
}
struct Part1;

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part2)]
}

fn test_input_data() -> anyhow::Result<&'static str> {
    Ok("Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3")
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Coord) -> u64 {
        TryInto::<u64>::try_into(self.x.abs_diff(other.x) + self.y.abs_diff(other.y)).unwrap()
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
struct Sensor {
    loc: Coord,
    closest_beacon: Coord,
}

#[derive(Debug)]
struct SensorParseError {
    msg: String,
}

impl SensorParseError {
    fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl From<std::num::ParseIntError> for SensorParseError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl Sensor {
    fn new(s: Coord, b: Coord) -> Self {
        Self {
            loc: s,
            closest_beacon: b,
        }
    }
}

impl std::str::FromStr for Sensor {
    type Err = SensorParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        match re.captures(s).map(|c| c.extract()) {
            Some((_, [sx, sy, bx, by])) => Ok(Sensor::new(
                Coord::new(sx.parse()?, sy.parse()?),
                Coord::new(bx.parse()?, by.parse()?),
            )),
            None => Err(SensorParseError::new(format!("parse failure: {}", s))),
        }
    }
}

fn span_for_line(sensors: &Vec<Sensor>, _beacons: &HashSet<Coord>, the_row: i64) -> Intervals {
    let mut intervals = Intervals::new();

    for s in sensors.iter() {
        let radius = s.loc.dist(&s.closest_beacon);

        let dy = TryInto::<i64>::try_into(radius).unwrap() - (s.loc.y - the_row).abs();
        // println!("Sensor at {}\tRadius: {}\tDistance: {}", s.loc, radius, dy);
        if dy >= 0 {
            let i: Interval = [s.loc.x - dy, s.loc.x + dy].into();
            //println!("merging {:?}", i);
            intervals.merge(i);
            //                let i: IntervalSet<i32> = (s.loc.x - dy, s.loc.x + dy).to_interval_set();
            //               is = is.union(&i);
            //println!("after merge: {:?}", intervals);
        }
    }
    intervals
}
impl Part1 {}

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        //test_input_data()
        crate::read_file("src/d15/input.txt")
    }

    fn run(&self, input: &str) -> String {
        let sensors: Vec<Sensor> = input
            .lines()
            .map(str::parse::<Sensor>)
            .collect::<Result<Vec<Sensor>, _>>()
            .unwrap();

        let beacons: HashSet<Coord> = sensors.iter().map(|s| s.closest_beacon).collect();
        let the_row: i64 = 2_000_000;
        let intervals = span_for_line(&sensors, &beacons, the_row);
        println!("{:?}", intervals);
        format!(
            "{}",
            intervals.span() - beacons.iter().filter(|c| c.y == the_row).count() as u32
        )
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        //test_input_data()
        crate::read_file("src/d15/input.txt")
    }

    fn run(&self, input: &str) -> String {
        let sensors: Vec<Sensor> = input
            .lines()
            .map(str::parse::<Sensor>)
            .collect::<Result<Vec<Sensor>, _>>()
            .unwrap();

        let beacons: HashSet<Coord> = sensors.iter().map(|s| s.closest_beacon).collect();
        for row in 0..=4_000_000 {
            let mut intervals = span_for_line(&sensors, &beacons, row);
            intervals.compress();
            if intervals.interval_count() > 1 {
                let col = intervals.intervals[0].high + 1;
                println!("row: {}, col: {}", row, col);
                return format!("{}", col * 4_000_000 + row);
            }
        }
        "none".to_string()
    }
}

#[cfg(test)]
mod test {

    use crate::d15::{Interval, Intervals};

    #[test]
    fn test_intervals() {
        let mut is: Intervals = vec![].into();
        let mut it = is.iter();
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![].into();
        is.merge([2, 4].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(2, 4)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![].into();
        is.merge([2, 2].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(2, 2)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[1, 4].into(), [7, 8].into()].into();
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(1, 4)));
        assert_eq!(it.next(), Some(&Interval::new(7, 8)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[1, 4].into(), [7, 8].into()].into();
        is.merge([3, 5].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(1, 5)));
        assert_eq!(it.next(), Some(&Interval::new(7, 8)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[1, 4].into(), [10, 13].into()].into();
        is.merge([6, 8].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(1, 4)));
        assert_eq!(it.next(), Some(&Interval::new(6, 8)));
        assert_eq!(it.next(), Some(&Interval::new(10, 13)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[4, 6].into(), [10, 13].into()].into();
        is.merge([1, 2].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(1, 2)));
        assert_eq!(it.next(), Some(&Interval::new(4, 6)));
        assert_eq!(it.next(), Some(&Interval::new(10, 13)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[4, 6].into(), [10, 13].into()].into();
        is.merge([1, 5].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(1, 6)));
        assert_eq!(it.next(), Some(&Interval::new(10, 13)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[4, 6].into(), [10, 13].into()].into();
        is.merge([2, 4].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(2, 6)));
        assert_eq!(it.next(), Some(&Interval::new(10, 13)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[4, 6].into(), [10, 13].into()].into();
        is.merge([2, 7].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(2, 7)));
        assert_eq!(it.next(), Some(&Interval::new(10, 13)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[4, 6].into(), [10, 13].into()].into();
        is.merge([6, 8].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(4, 8)));
        assert_eq!(it.next(), Some(&Interval::new(10, 13)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[4, 6].into(), [10, 13].into()].into();
        is.merge([6, 10].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(4, 13)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[4, 6].into(), [10, 13].into()].into();
        is.merge([6, 12].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(4, 13)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[4, 6].into(), [10, 13].into()].into();
        is.merge([4, 20].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(4, 20)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[4, 6].into(), [10, 13].into()].into();
        is.merge([1, 20].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(1, 20)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[4, 6].into(), [10, 13].into()].into();
        is.merge([8, 9].into());
        is.compress();
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(4, 6)));
        assert_eq!(it.next(), Some(&Interval::new(8, 13)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[-4, 2].into(), [10, 13].into()].into();
        is.merge([5, 7].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(-4, 2)));
        assert_eq!(it.next(), Some(&Interval::new(5, 7)));
        assert_eq!(it.next(), Some(&Interval::new(10, 13)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![[-4, 2].into(), [10, 13].into()].into();
        is.merge([5, 11].into());
        let mut it = is.iter();
        assert_eq!(it.next(), Some(&Interval::new(-4, 2)));
        assert_eq!(it.next(), Some(&Interval::new(5, 13)));
        assert_eq!(it.next(), None);

        let mut is: Intervals = vec![].into();
        is.merge([343706, 967194].into());
        is.merge([1203528, 2794988].into());
        is.merge([2392879, 5588065].into());
        is.merge([-138831, 516047].into());
        is.merge([-837068, 967194].into());
        is.merge([967194, 2489994].into());
        is.merge([967194, 1345520].into());
        is.merge([3477400, 3882064].into());
        is.merge([826422, 967194].into());
        is.merge([1611175, 3652823].into());
    }

    #[test]
    fn interval_span() {
        let mut is: Intervals = vec![[-4, 2].into(), [10, 13].into()].into();
        assert_eq!(is.span(), 11);

        let mut is: Intervals = vec![].into();
        assert_eq!(is.span(), 0);

        let mut is: Intervals = vec![[2, 2].into()].into();
        assert_eq!(is.span(), 1);
    }
}
