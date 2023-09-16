use crate::d14::topo::{parse_lines, Coord, State, StepResult, Topo};
use crate::PuzzleRun;

mod topo;
struct Part1;

struct Part2;

pub fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part2)]
}

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        crate::read_file("src/d14/input-test.txt")
    }

    fn run(&self, input: &str) -> String {
        let mut topo = Topo::from_lines(parse_lines(input.lines()).unwrap());
        let mut grain_count = 0;
        topo.drop_at(Coord::new(500, 0));

        loop {
            let results = topo.step();
            if results.len() != 1 {
                panic!();
            }
            match results[0] {
                StepResult::Moved(from, to) => {}
                StepResult::Stopped(p) => {
                    if !topo.drop_at(Coord::new(500, 0)) {
                        panic!()
                    }
                    grain_count += 1;
                }
                StepResult::Off(p) => {
                    return format!("{}", grain_count);
                }
            }
        }
    }
}

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        crate::read_file("src/d14/input.txt")
    }

    fn run(&self, input: &str) -> String {
        let mut topo = Topo::from_lines(parse_lines(input.lines()).unwrap());
        topo.with_floor();
        let mut grain_count = 1;
        topo.drop_at(Coord::new(500, 0));

        loop {
            let results = topo.step();
            if results.len() != 1 {
                panic!();
            }
            match results[0] {
                StepResult::Moved(from, to) => {}
                StepResult::Stopped(p) => {
                    if !topo.drop_at(Coord::new(500, 0)) {
                        return format!("{}", grain_count);
                    }
                    grain_count += 1;
                }
                StepResult::Off(p) => {
                    panic!("This should never happen");
                }
            }
        }
    }
}
