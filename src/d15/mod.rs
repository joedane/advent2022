use crate::PuzzleRun;
use anyhow::Result;
use gcollections::ops::*;

use interval::interval_set::ToIntervalSet;
use interval::ops::Range;
use interval::{Interval, IntervalSet};

use num_traits::*;
use regex::Regex;

use std::collections::HashSet;

struct Part1;

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
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
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Coord) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
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

        let mut is: IntervalSet<i32> = IntervalSet::empty();
        let beacons: HashSet<Coord> = sensors.iter().map(|s| s.closest_beacon).collect();

        let the_row = 10;

        println!("read {} sensors:", sensors.len());
        for s in sensors.iter() {
            let radius = s.loc.dist(&s.closest_beacon);

            let mut dy = radius as i32 - (s.loc.y - the_row).abs();
            println!("Processing senor at {} with distance {}", s.loc, dy);
            if radius >= (the_row - s.loc.y).abs() as u32 {
                let i: IntervalSet<i32> = (s.loc.x - dy, s.loc.x + dy).to_interval_set();
                is = is.union(&i);
            }
        }
        format!(
            "{}",
            is.size() - beacons.iter().filter(|c| c.y == the_row).count() as u32
        )
    }
}
