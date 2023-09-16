use crate::topo::{State, Topo};
use crate::Coord;
use egui::{
    pos2, Align, Align2, Color32, Label, Layout, Painter, Rect, ScrollArea, Stroke, TextStyle,
};
use emath::RectTransform;

pub struct App {
    topo: Topo,
    cell_size: f32,
    border_size: f32,
    updates_per_second: u32,
    spawn_rate: f64,
    last_updated: f64,
    last_spawn: f64,
    running: bool,
    current_pointer_pos: String,
}

impl App {
    pub(crate) fn new(_cc: &eframe::CreationContext<'_>, topo: Topo) -> App {
        App {
            topo,
            cell_size: 15.0,
            border_size: 1.0,
            updates_per_second: 2,
            spawn_rate: 0.0,
            last_updated: 0.0,
            last_spawn: 0.0,
            running: true,
            current_pointer_pos: "(-, -)".to_string(),
        }
    }

    fn canvas_width(&self) -> f32 {
        self.topo.get_width() as f32 * self.cell_size
    }

    fn canvas_height(&self) -> f32 {
        self.topo.get_height() as f32 * self.cell_size
    }

    fn get_x_offset(&self) -> f32 {
        self.topo.get_x_offset() as f32 * self.cell_size
    }

    fn draw_grid(&self, painter: &Painter, xf: &RectTransform, viewport: Rect) {
        let stroke = Stroke::new(self.border_size / 3.0, Color32::RED);
        let stroke_bold = Stroke::new(self.border_size, Color32::RED);
        // as f32 * self.cell_size;
        let bounds = self.topo.get_bounds();

        for i in bounds.upper_left.y..=bounds.lower_right.y {
            painter.line_segment(
                [
                    xf.transform_pos(pos2(0., i as f32 * self.cell_size)),
                    xf.transform_pos(pos2(self.canvas_width(), i as f32 * self.cell_size)),
                ],
                if i % 10 == 0 { stroke_bold } else { stroke },
            );
        }

        for i in bounds.upper_left.x..=bounds.lower_right.x {
            painter.line_segment(
                [
                    xf.transform_pos(pos2(i as f32 * self.cell_size, 0.0)),
                    xf.transform_pos(pos2(i as f32 * self.cell_size, viewport.height())),
                ],
                if i % 10 == 0 { stroke_bold } else { stroke },
            );
        }
    }

    fn draw_world(&self, painter: &egui::Painter, font_id: &egui::FontId, xf: &RectTransform) {
        let mut used_rect = Rect::NOTHING;

        for (c, state) in self.topo.coord_iter() {
            match state {
                State::Wall => {
                    let text_rect = self.draw_cell(painter, xf, font_id, c, '#');
                    used_rect = used_rect.union(text_rect);
                }
                State::Sand => {
                    let text_rect = self.draw_cell(painter, xf, font_id, c, '*');
                    used_rect = used_rect.union(text_rect);
                }
                _ => {}
            }
        }
        //       ui.allocate_rect(used_rect, Sense::hover());
    }

    fn draw_cell(
        &self,
        painter: &egui::Painter,
        xf: &RectTransform,
        font_id: &egui::FontId,
        coord: Coord,
        glyph: char,
    ) -> Rect {
        // println!("drawing {} at {:?}", glyph, coord);
        painter.text(
            xf.transform_pos(pos2(
                self.cell_size * coord.x as f32 + (self.cell_size / 2.0),
                self.cell_size * coord.y as f32 + (self.cell_size / 2.0),
            )),
            Align2::CENTER_CENTER,
            glyph,
            font_id.clone(),
            Color32::WHITE,
        )
    }

    fn update_world(&mut self, try_drop: bool) {
        if try_drop && !self.topo.drop_at(Coord::new(500, 0)) {
            println!("SFSFFSF");
            return;
        }
        let results = self.topo.step();
        if results.len() == 0 {
            println!("No results");
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //        let (grid_width, grid_height) = (500u32, 500u32);

        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.running = !self.running;
        }

        let now = ctx.input(|i| i.time);
        if self.running && now > self.last_updated + (1.0 / self.updates_per_second as f64) {
            self.update_world(false);
            self.last_updated = now; // what difference if I update here vs after painting?
        }

        egui::TopBottomPanel::top("control_panel")
            .resizable(false)
            .min_height(32.)
            .show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.add(Label::new(&self.current_pointer_pos));
                    ui.add(
                        egui::Slider::new(&mut self.updates_per_second, 1..=200).text("interval"),
                    );
                    if ui.button("Button 1").clicked() {
                        println!("Button 1");
                    }
                    if ui.button("Drop").clicked() {
                        self.topo.drop_at(Coord::new(500, 0));
                        self.last_spawn = now;
                    }
                })
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both()
                .auto_shrink([false; 2])
                .show_viewport(ui, |ui, viewport| {
                    ui.set_height(self.canvas_height().max(viewport.height()));
                    ui.set_width(self.canvas_width().max(viewport.width()));

                    //ui.set_height(self.canvas_height());
                    //ui.set_width(self.canvas_width());
                    let (response, painter) = ui.allocate_painter(
                        egui::Vec2::new(ui.available_width(), ui.available_height()),
                        egui::Sense::hover(),
                    );

                    let to_screen = RectTransform::from_to(
                        Rect::from_min_size(
                            pos2(viewport.left() + self.get_x_offset(), viewport.left_top().y),
                            response.rect.size(),
                        ),
                        response.rect,
                    );

                    let from_screen = to_screen.inverse();

                    ctx.input(|i| {
                        for e in &i.events {
                            if let egui::Event::PointerMoved(p) = e {
                                let p = from_screen.transform_pos(*p);
                                self.current_pointer_pos = format!(
                                    "({}, {})",
                                    (p.x / self.cell_size).floor() as usize,
                                    (p.y / self.cell_size).floor() as usize
                                );
                            }
                        }
                    });

                    // let to_screen = RectTransform::from_to(
                    //     response.rect.translate(viewport.left_top().to_vec2()),
                    //     response.rect,
                    // );

                    self.draw_grid(&painter, &to_screen, viewport);
                    let font_id = TextStyle::Body.resolve(ui.style());

                    self.draw_world(&painter, &font_id, &to_screen);
                });
        });

        ctx.request_repaint_after(core::time::Duration::from_secs_f64(
            1.0 / self.updates_per_second as f64,
        ));
    }
}
