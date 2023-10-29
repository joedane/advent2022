use crate::PuzzleRun;
use regex::Regex;
use std::collections::HashMap;

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    rate: u32,
    duration: u32,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(name: String, rate: u32, tunnels: Vec<String>) -> Self {
        Self {
            name,
            rate,
            duration: 0,
            tunnels,
        }
    }
}

#[derive(Debug)]
struct ValveParseError {
    msg: String,
}

impl std::str::FromStr for Valve {
    type Err = ValveParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Valve ([[:alpha:]]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)$",
        )
        .unwrap();
        match re.captures(s).map(|c| c.extract()) {
            Some((_, [name, rate, tunnels])) => Ok(Valve::new(
                name.into(),
                u32::from_str(rate).unwrap(),
                tunnels.split(',').map(|s| s.trim().to_owned()).collect(),
            )),
            None => Err(ValveParseError {
                msg: format!("failed to parse: {}", s),
            }),
        }
    }
}

impl core::fmt::Display for ValveParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Clone, Debug)]
struct WorldState {
    valves: HashMap<String, Valve>,
    at: String,
    time_remaining: u32,
}

struct NextStateIter {
    base: WorldState,
    checked_my_valve: bool,
    tunnels: Vec<String>,
}

impl NextStateIter {
    fn new(base: &WorldState) -> Self {
        let tunnels = base.valves.get(&base.at).unwrap().tunnels.clone();

        Self {
            base: base.clone(),
            checked_my_valve: false,
            tunnels,
        }
    }
}

impl Iterator for NextStateIter {
    type Item = WorldState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.base.time_remaining == 0 {
            println!("from {} END", self.base.at);
            return None;
        }

        let mut n = self.base.clone();
        n.time_remaining -= 1;
        if !self.checked_my_valve {
            self.checked_my_valve = true;
            let at = n.valves.get_mut(&n.at).unwrap();
            if at.duration == 0 && at.rate > 0 {
                at.duration = n.time_remaining;
                //println!("from {} set valve, duration {}", self.base.at, at.duration);
                return Some(n);
            }
        }

        if !self.tunnels.is_empty() {
            let t = self.tunnels.pop().unwrap();
            n.at = t;
            n.remove_tunnel(&self.base.at, &n.at.clone());
            //println!("from {} move to {}", self.base.at, n.at);
            return Some(n);
        } else {
            //println!("from {} no children", self.base.at);
            return None;
        }
    }
}

impl WorldState {
    fn init<'a, T: Iterator<Item = &'a str>>(input: T) -> Result<Self, ValveParseError> {
        let valves = input
            .map(|s| {
                let v = s.parse::<Valve>()?;
                Ok((v.name.clone(), v))
            })
            .collect::<Result<HashMap<String, Valve>, ValveParseError>>()?;
        Ok(Self {
            valves,
            at: "AA".to_string(),
            time_remaining: 30,
        })
    }

    fn next_state_iter(&self) -> NextStateIter {
        NextStateIter::new(self)
    }

    fn total_flow(&self) -> u32 {
        self.valves.values().map(|v| v.duration * v.rate).sum()
    }

    fn remove_tunnel(&mut self, from: &str, to: &str) {
        let v = self.valves.get_mut(from).unwrap();
        v.tunnels.retain(|t| t != to);
    }
    fn print_debug(&self, level: usize) {
        let mut s = String::new();
        for i in 0..level {
            s.push(' ');
        }
        s.push_str("WS at: ");
        s.push_str(&self.at);
        s.push_str(" [");
        s.push_str(&format!("{:p}", self));
        s.push_str("] (rem: ");
        s.push_str(&format!("{}", self.time_remaining));
        s.push_str(") ");
        for v in self.valves.values() {
            s.push_str(&v.name);
            s.push_str(": [");
            for t in &v.tunnels {
                s.push_str(&t);
                s.push(',');
            }
            s.push_str("] ");
        }
        println!("{}", s);
    }

    fn best(&self, level: usize) -> Option<WorldState> {
        let mut best_value = std::u32::MIN;
        let mut best_state: Option<WorldState> = None;
        //self.print_debug(level);
        //println!("[{:p}] {:?}", self, self);
        for s in self.next_state_iter() {
            if let Some(this_best) = s.best(level + 1) {
                let total_flow = this_best.total_flow();
                //println!("BEST: ");
                //this_best.print_debug(level);
                if total_flow > best_value {
                    best_value = total_flow;
                    best_state = Some(this_best);
                }
            }
        }
        best_state
    }
}
struct Part1;

fn test_data() -> &'static str {
    "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II"
}

fn simple_test_data() -> &'static str {
    "Valve AA has flow rate=0; tunnels lead to valves BB, CC
    Valve BB has flow rate=13; tunnel leads to valves CC
    Valve CC has flow rate=2; tunnel leads to valve AA"
}

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        Ok(test_data())
    }

    fn run(&self, input: &str) -> String {
        let state = WorldState::init(input.lines()).unwrap();
        format!("{}", state.best(0).unwrap().total_flow())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use regex::Regex;

    #[test]
    fn test_parse() {
        //let re_str = r"Valve ([[:alpha:]]{2}) has flow rate=(\d+); tunnels? lead to valves? (.+)$";
        let re_str = r"Valve ([[:alpha:]]{2}) has flow rate=(\d+); tunnels? leads? to valves?";

        let re = Regex::new(re_str).unwrap();
        for l in super::test_data().lines() {
            match re.captures(l) {
                Some(c) => {
                    println!("MATCHED: {:?}", c);
                }
                None => {
                    println!("FAIL: {}", l);
                }
            }
        }
    }

    #[test]
    fn test_world_iter() {
        let ws = WorldState::init(test_data().lines()).unwrap();
        let mut wsi = ws.next_state_iter();
        let next = wsi.next();
        assert!(next.is_some());
        if let Some(ws) = next {
            assert_eq!(ws.time_remaining, 29);
            assert_eq!(ws.at, "BB");
        }
    }

    #[test]
    fn test_part1() {
        let p1 = Part1;
        p1.run(p1.input_data().unwrap());
    }
}
