use anyhow::{anyhow, Result};

mod app;
mod topo;

use topo::{Coord, Line};

fn parse_line(input: &str) -> Result<Line> {
    input
        .split("->")
        .map(|s| s.trim())
        .map(|s| match s.find(',') {
            Some(i) => Ok(Coord {
                x: s[0..i].parse().unwrap(),
                y: s[i + 1..].parse().unwrap(),
            }),
            None => Err(anyhow!("bad coordinate: {s}")),
        })
        .collect()
}

fn parse_lines<T>(lines: T) -> Result<Vec<Line>>
where
    T: Iterator<Item = &'static str>,
{
    lines
        .into_iter()
        .map(|s| s.trim())
        .map(|l| parse_line(l))
        .collect::<Result<Vec<Line>, _>>()
}

fn main() -> Result<()> {
    use app::App;
    use emath::vec2;
    use topo::Topo;

    let input = include_str!("input.txt");

    let topo = Topo::from_lines(parse_lines(input.lines())?);
    //let topo: Topo = (0..20).map(|i| Coord::new(i, i)).collect();

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(800., 800.)),
        ..Default::default()
    };

    eframe::run_native(
        "FOOBAR",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, topo))),
    )
    .map_err(|e| anyhow!("failed to start app: {}", e.to_string()))?; // gotta be a better way to do this
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::topo::{State, Topo};

    #[test]
    fn test_parse() {
        let line = parse_line("508,146 -> 513,146").unwrap();
        assert_eq!(
            line,
            Line {
                points: vec![Coord::new(508, 146), Coord::new(513, 146)]
            }
        );
    }

    #[test]
    fn test_print() {
        let input = "498,4 -> 498,6 -> 496,6
                503,4 -> 502,4 -> 502,9 -> 494,9";
        let topo = Topo::from_lines(parse_lines(input.lines()).unwrap());
        println!("{topo:?}");
    }

    #[test]
    fn test_step() {
        let input = "498,4 -> 498,6 -> 496,6
                503,4 -> 502,4 -> 502,9 -> 494,9";
        let topo = Topo::from_lines(parse_lines(input.lines()).unwrap());
        let mut c = Coord::new(500, 0);
        assert_eq!(topo.drop_grain(c), Some(Coord::new(500, 8)));
    }

    #[test]
    fn test_steps() {
        let input = "498,4 -> 498,6 -> 496,6
                503,4 -> 502,4 -> 502,9 -> 494,9";
        let mut topo = Topo::from_lines(parse_lines(input.lines()).unwrap());
        let c = Coord::new(500, 0);
        let mut i = 1;
        loop {
            println!("{}\n{:?}", i, topo);
            i += 1;
            match topo.drop_grain(c) {
                None => break,
                Some(to) => {
                    topo[to] = State::Sand;
                }
            }
        }
    }

    #[test]
    fn test_print_large() {
        let input = include_str!("input.txt");
        let topo = Topo::from_lines(parse_lines(input.lines()).unwrap());
        println!("{topo:?}");
    }
}
