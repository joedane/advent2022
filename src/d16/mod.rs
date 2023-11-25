use crate::PuzzleRun;
use nom::AsBytes;
use petgraph::{
    algo::BoundedMeasure,
    data::DataMap,
    graph::{EdgeReference, Graph, NodeIndex, UnGraph},
    visit::{Data, EdgeRef, GraphBase, IntoEdgesDirected, NodeIndexable},
    Direction, IntoWeightedEdge,
};
use regex::Regex;
use std::collections::HashMap;
use tracing::{event, info, instrument, Level};

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Debug, Clone)]
struct Valve {
    name: [u8; 2],
    rate: u32,
    tunnels: Vec<[u8; 2]>,
}

impl Valve {
    fn new(name_in: &[u8], rate: u32, tunnels: Vec<[u8; 2]>) -> Self {
        let mut name: [u8; 2] = Default::default();
        name.copy_from_slice(name_in);
        Self {
            name,
            rate,
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
                &name.as_bytes()[0..2],
                u32::from_str(rate).unwrap(),
                tunnels
                    .split(',')
                    .map(|s| {
                        let mut a: [u8; 2] = Default::default();
                        a.clone_from_slice(&s.trim()[0..2].as_bytes());
                        a
                    })
                    .collect(),
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

type NodeId = <Graph<ValveState, ()> as GraphBase>::NodeId;
type CostType = HashMap<(NodeId, NodeId), u32>;

/*
#[derive(Clone, Debug)]
struct WorldState<'a, K>
where
    K: BoundedMeasure,
{
    graph: Graph<&'a str, ()>,
    costs: CostType<'a, K>,
}
*/

#[derive(Debug)]
enum PathStep {
    OpenValve {
        time: u32,
        rate: u32,
        flow_at_this_step: u32,
        valve_opened: NodeId,
        next: Box<PathStep>,
    },
    StepTo {
        time: u32,
        flow_at_this_step: u32,
        step_to: NodeId,
        next: Box<PathStep>,
    },
    Complete,
}

impl PathStep {
    fn flow_at_completion(&self) -> u32 {
        let mut flow: u32 = 0;
        let mut step = self;
        loop {
            match step {
                PathStep::Complete => return flow,
                PathStep::StepTo {
                    time,
                    step_to,
                    flow_at_this_step,
                    next,
                } => {
                    flow += flow_at_this_step;
                    step = next;
                }
                PathStep::OpenValve {
                    time,
                    flow_at_this_step,
                    rate,
                    valve_opened,
                    next,
                } => {
                    flow += flow_at_this_step;
                    step = next;
                }
            }
        }
        flow
    }

    fn dump(&self) {
        match self {
            PathStep::Complete => {
                println!("completed");
            }
            PathStep::StepTo {
                time,
                flow_at_this_step,
                step_to,
                next,
            } => {
                println!(" move to {:?}", step_to);
                next.dump();
            }
            PathStep::OpenValve {
                time,
                flow_at_this_step,
                rate,
                valve_opened,
                next,
            } => {
                println!("opened valve at {:?} rate {}", valve_opened, rate,);
                next.dump();
            }
        }
    }
}

fn init_valves<'a, T: Iterator<Item = &'a str>>(input: T) -> HashMap<[u8; 2], Valve> {
    input
        .map(|s| {
            let v = s.parse::<Valve>()?;
            Ok((v.name.clone(), v))
        })
        .collect::<Result<HashMap<[u8; 2], Valve>, ValveParseError>>()
        .unwrap()
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct ValveState {
    name: [u8; 2],
    rate: u32,
    opened_time: u32,
}

impl ValveState {
    fn init(valve: &Valve) -> Self {
        Self {
            name: valve.name,
            rate: valve.rate,
            opened_time: 0,
        }
    }
}

impl std::fmt::Display for ValveState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ValveState [at: {}, rate: {}, opened: {}",
            std::str::from_utf8(self.name.as_bytes()).unwrap(),
            self.rate,
            self.opened_time
        )
    }
}

#[derive(Clone, Debug)]
struct WorldState {
    at: NodeId,
    valves: HashMap<NodeId, ValveState>,
}

impl WorldState {
    fn init<G>(start_from: NodeId, graph: &G, nodes: &HashMap<(NodeId, NodeId), u32>) -> Self
    where
        G: NodeIndexable + DataMap,
        G: GraphBase<NodeId = NodeIndex>,
        G: Data<NodeWeight = ValveState>,
    {
        Self {
            at: start_from,
            valves: nodes
                .keys()
                .flat_map(|(f, t)| {
                    vec![
                        (*f, *graph.node_weight(*f).unwrap()),
                        (*t, *graph.node_weight(*t).unwrap()),
                    ]
                })
                .collect(),
        }
    }
    fn turn_on_current_valve(&self, time: u32) -> Self {
        let mut new_w = self.clone();
        let v = new_w.valves.get_mut(&self.at).unwrap();
        if v.opened_time > 0 {
            panic!();
        }
        v.opened_time = time;
        new_w
    }

    fn take_tunnel(&self, to: NodeId) -> Self {
        let mut new_w = self.clone();
        new_w.at = to;
        new_w
    }

    fn get_valve(&self, id: NodeId) -> ValveState {
        *self.valves.get(&id).unwrap()
    }

    fn get_current_valve(&self) -> ValveState {
        self.get_valve(self.at)
    }

    fn is_complete(&self) -> bool {
        self.valves
            .values()
            .all(|v| v.rate == 0 || v.opened_time > 0)
    }

    fn flow_per_step(&self) -> u32 {
        self.valves.values().map(|v| v.rate as u32).sum()
    }
}

fn has_tunnel(graph: &Graph<ValveState, ()>, from: NodeId, to: NodeId) -> bool {
    graph.contains_edge(
        NodeIndex::new(graph.to_index(from)),
        NodeIndex::new(graph.to_index(to)),
    )
}

fn get_tunnels_from(
    graph: &Graph<ValveState, ()>,
    costs: &CostType,
    from: NodeId,
) -> Vec<(NodeId, u32)> {
    costs
        .iter()
        .filter_map(|((f, t), cost)| {
            if *f == from && *t != from {
                Some((*t, *cost))
            } else {
                None
            }
        })
        .collect()
}

fn best(
    graph: &Graph<ValveState, ()>,
    costs: &CostType,
    world_state: WorldState,
    time_at_start: u32,
    time_limit: u32,
) -> Option<PathStep> {
    if time_at_start >= time_limit {
        return if world_state.is_complete() {
            Some(PathStep::Complete)
        } else {
            None
        };
    }
    event!(Level::INFO, time = time_at_start, at = %graph[world_state.at]);

    let mut best_value = std::u32::MIN;
    let mut best_step: Option<PathStep> = None;

    let this_valve = world_state.get_current_valve();
    if this_valve.opened_time == 0 && this_valve.rate > 0 {
        let next = world_state.turn_on_current_valve(time_at_start + 1);
        if let Some(step) = best(graph, costs, next, time_at_start + 1, time_limit) {
            event!(Level::INFO, "turning on valve at {}", this_valve);
            let v = world_state.get_current_valve();
            best_value = step.flow_at_completion();
            best_step.replace(PathStep::OpenValve {
                time: time_at_start + 1,
                flow_at_this_step: world_state.flow_per_step(),
                rate: v.rate,
                valve_opened: world_state.at,
                next: Box::new(step),
            });
        }
    }

    //self.print_debug(level);
    //println!("[{:p}] {:?}", self, self);
    for (t, cost) in get_tunnels_from(graph, costs, world_state.at) {
        event!(
            Level::INFO,
            "tring a move from {} to {}",
            graph[world_state.at],
            graph[t]
        );
        let next = world_state.take_tunnel(t);
        if let Some(step) = best(graph, costs, next, time_at_start + cost as u32, time_limit) {
            let this_flow = step.flow_at_completion();
            if this_flow > best_value {
                best_step.replace(PathStep::StepTo {
                    time: time_at_start + cost,
                    flow_at_this_step: world_state.flow_per_step(),
                    step_to: t.clone(),
                    next: Box::new(step),
                });
                best_value = this_flow;
            }
        }
    }

    if best_step.is_some() {
        best_step
    } else if world_state.is_complete() {
        Some(PathStep::Complete)
    } else {
        None
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
        Ok(simple_test_data())
    }

    fn run(&self, input: &str) -> String {
        let valves = init_valves(input.lines());
        let mut graph = Graph::<ValveState, ()>::new();

        let nodes: HashMap<[u8; 2], NodeIndex<_>> = valves
            .iter()
            .map(|(k, v)| (*k, graph.add_node(ValveState::init(v))))
            .collect();

        let start_node: NodeId = graph.from_index(nodes.get(b"AA").unwrap().index());

        graph.extend_with_edges(nodes.iter().flat_map(|(id, &from_node)| {
            let nodes = &nodes;
            valves
                .get(id)
                .unwrap()
                .tunnels
                .iter()
                .map(move |t| (from_node, *nodes.get(t).unwrap()))
            //                    .map(move |t| (from_node, map_edge(t.as_str(), nodes)))
        }));

        let mut costs: HashMap<_, _> = petgraph::algo::floyd_warshall(&graph, |e| 1).unwrap();
        costs.retain(|(from, to), _| {
            *from == start_node || (graph[*from].rate > 0 && graph[*to].rate > 0)
        });

        match best(
            &graph,
            &costs,
            WorldState::init(start_node, &graph, &costs),
            0,
            30,
        ) {
            Some(path) => {
                path.dump();
                "OK".to_string()
            }
            None => "failed".to_string(),
        }
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

    fn setup() {
        tracing_subscriber::fmt::init();
    }

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
        setup();
        let p1 = Part1;
        p1.run(p1.input_data().unwrap());
    }
}
