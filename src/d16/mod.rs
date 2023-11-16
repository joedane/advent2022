use crate::PuzzleRun;
use petgraph::{
    graph::{Graph, NodeIndex, UnGraph},
    IntoWeightedEdge,
};
use regex::Regex;
use std::collections::HashMap;

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

const TOTAL_TIME: u32 = 30;

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

#[derive(Debug)]
enum PathStep {
    OpenValve {
        total_flow: u32,
        time_remaining: u32,
        duration: u32,
        rate: u32,
        at: String,
        next: Box<PathStep>,
    },
    StepTo {
        total_flow: u32,
        time_remaining: u32,
        step_to: String,
        next: Box<PathStep>,
    },
    Complete(u32),
}

impl PathStep {
    fn total_flow(&self) -> u32 {
        match self {
            PathStep::Complete(v) => *v,
            PathStep::StepTo {
                total_flow,
                time_remaining: _,
                step_to: _,
                next: _,
            } => *total_flow,
            PathStep::OpenValve {
                total_flow,
                time_remaining: _,
                duration: _,
                rate: _,
                at: _,
                next: _,
            } => *total_flow,
        }
    }

    fn dump(&self) {
        match self {
            PathStep::Complete(v) => {
                println!("completed with total flow {}", v);
            }
            PathStep::StepTo {
                total_flow,
                time_remaining,
                step_to,
                next,
            } => {
                println!(
                    "at time {} move to {}",
                    TOTAL_TIME - time_remaining + 1,
                    step_to
                );
                next.dump();
            }
            PathStep::OpenValve {
                total_flow,
                time_remaining,
                duration,
                rate,
                at,
                next,
            } => {
                println!(
                    "at time {} opened valve at {} rate {} for duration {}",
                    TOTAL_TIME - time_remaining + 1,
                    at,
                    rate,
                    duration
                );
                next.dump();
            }
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

    fn turn_on_current_valve(&self) -> WorldState {
        let mut new_w = self.clone();
        let v = new_w.valves.get_mut(&new_w.at).unwrap();
        if v.duration > 0 {
            panic!();
        }
        v.duration = self.time_remaining - 1;
        new_w.time_remaining -= 1;
        new_w
    }

    fn take_tunnel(&self, to: &str) -> WorldState {
        let mut new_w = self.clone();
        let mut v = new_w.valves.get_mut(&new_w.at).unwrap();
        if !v.tunnels.iter().any(|x| x == to) {
            panic!();
        }
        new_w.time_remaining -= 1;
        new_w.at = to.to_string();
        new_w
    }

    fn total_flow(&self) -> u32 {
        self.valves.values().map(|v| v.duration * v.rate).sum()
    }

    fn is_complete(&self) -> bool {
        self.valves.values().all(|v| v.rate == 0 || v.duration > 0)
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

    fn best(&self, level: usize) -> Option<PathStep> {
        if self.time_remaining == 0 {
            return if self.is_complete() {
                Some(PathStep::Complete(self.total_flow()))
            } else {
                None
            };
        }

        let mut best_value = std::u32::MIN;
        let mut best_step: Option<PathStep> = None;

        let this_valve = self.valves.get(&self.at).unwrap();
        if this_valve.duration == 0 && this_valve.rate > 0 {
            let next = self.turn_on_current_valve();
            if let Some(step) = next.best(level + 1) {
                let v = next.valves.get(&self.at).unwrap();
                best_value = step.total_flow();
                best_step = Some(PathStep::OpenValve {
                    total_flow: step.total_flow(),
                    time_remaining: self.time_remaining,
                    rate: v.rate,
                    duration: v.duration,
                    at: self.at.clone(),
                    next: Box::new(step),
                });
            }
        }

        //self.print_debug(level);
        //println!("[{:p}] {:?}", self, self);
        for t in &this_valve.tunnels[..] {
            let next = self.take_tunnel(t);
            if let Some(step) = next.best(level + 1) {
                let this_flow = step.total_flow();
                if this_flow > best_value {
                    best_step.replace(PathStep::StepTo {
                        total_flow: step.total_flow(),
                        time_remaining: self.time_remaining,
                        step_to: t.clone(),
                        next: Box::new(step),
                    });
                    best_value = this_flow;
                }
            }
        }

        if best_step.is_some() {
            best_step
        } else if self.is_complete() {
            Some(PathStep::Complete(self.total_flow()))
        } else {
            None
        }
    }

    fn to_graph(&self) -> UnGraph<&str, ()> {
        /*
        UnGraph::<&str, ()>::from_edges(
            self.valves
                .values()
                .flat_map(|v| v.tunnels.iter().map(|t| (v.name.as_str(), t.as_str()))),
        )
        */
        todo!()
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

fn vec_set(v: &mut Vec<u64>, row_size: usize, i: usize, j: usize, value: u64) {
    v[i * row_size + j] = value;
}

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        Ok(test_data())
    }

    fn run(&self, input: &str) -> String {
        let state = WorldState::init(input.lines()).unwrap();
        let mut graph = Graph::<&str, ()>::new();

        let nodes: HashMap<&str, NodeIndex<_>> = state
            .valves
            .keys()
            .map(|s| (s.as_str(), graph.add_node(s)))
            .collect();

        let edges: Vec<(NodeIndex<_>, NodeIndex<_>)> = nodes
            .iter()
            .flat_map(|(&id, &from_node)| {
                let nodes = &nodes;
                state
                    .valves
                    .get(id)
                    .unwrap()
                    .tunnels
                    .iter()
                    .map(move |t| (from_node, *nodes.get(t.as_str()).unwrap()))
                //                    .map(move |t| (from_node, map_edge(t.as_str(), nodes)))
            })
            .collect();
        graph.extend_with_edges(edges.iter());

        "OK".to_string()
    }
}

// delete
fn map_edge<'a, E>(to_node: &'a str, nodes: &'a HashMap<&str, NodeIndex<E>>) -> &'a NodeIndex<E> {
    nodes.get(to_node).unwrap()
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
    fn test_part1() {
        let p1 = Part1;
        p1.run(p1.input_data().unwrap());
    }
}
