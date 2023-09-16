use anyhow::{anyhow, Result};

mod app;
mod topo;

use topo::{parse_lines, Coord};

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
