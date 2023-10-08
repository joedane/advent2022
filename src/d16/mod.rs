use crate::PuzzleRun;
use regex::Regex;

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Debug)]
struct Valve {
    name: String,
    rate: u8,
    duration: u8,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(name: String, rate: u8, tunnels: Vec<String>) -> Self {
        Self {
            name,
            rate,
            duration: 0,
            tunnels,
        }
    }
}

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
                u8::from_str(rate).unwrap(),
                tunnels.split(',').map(|s| s.to_owned()).collect(),
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

struct WorldState {
    valves: Vec<Valve>,
    at: String,
    score: u32,
}

impl WorldState {

    fn init<T: Iterator<Item = Result<String, std::io::Error>>>(input: T) -> Result<Self, ValveParseError> {
        let valves: Vec<Valve> = input.map(|s| s.and_then(|s| s.parse::<Valve>())).collect();
        Ok(Self {
            valves,
            at: "AA".to_string(),
            score: 0
        })
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
impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        Ok(test_data())
    }

    fn run(&self, input: &str) -> String {
        let state: = input.lines().map(|s| s.parse::<Value>())
        for line in input.lines() {
            match line.parse::<Valve>() {
                Ok(v) => {
                    println!("Parsed {:?}", v)
                }
                Err(e) => {
                    println!("{}", e)
                }
            }
        }
        "OK".to_string()
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
}
