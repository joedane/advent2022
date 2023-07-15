use egui::{
    pos2, Align, Align2, Color32, Label, Layout, Painter, Rect, Response, ScrollArea, Sense,
    Stroke, TextStyle,
};
use emath::{RectTransform, Vec2};
use macroquad::prelude::{
    clear_background, draw_line, is_key_pressed, next_frame, screen_height, screen_width,
    set_camera, Camera2D, Conf, KeyCode, GRAY, RED,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "JAD Advent 2022".to_string(),
        window_width: 1600,
        window_height: 1400,
        ..Default::default()
    }
}

struct Grid {
    width: usize,
    height: usize,
    cell_size: f32,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cell_size: 15.0,
        }
    }

    fn canvas_width(&self) -> f32 {
        self.width as f32 * self.cell_size
    }

    fn canvas_height(&self) -> f32 {
        self.height as f32 * self.cell_size
    }
    fn draw_grid(&self, painter: &Painter, xf: &RectTransform) {
        let stroke = Stroke::new(0.3, Color32::RED);
        let stroke_bold = Stroke::new(1.0, Color32::RED);
        for i in 0..self.height {
            painter.line_segment(
                [
                    xf.transform_pos(pos2(self.cell_size, (i + 1) as f32 * self.cell_size)),
                    xf.transform_pos(pos2(self.canvas_width(), (i + 1) as f32 * self.cell_size)),
                ],
                if i % 10 == 0 { stroke_bold } else { stroke },
            );
        }

        for i in 0..self.width {
            painter.line_segment(
                [
                    xf.transform_pos(pos2((i + 1) as f32 * self.cell_size, self.cell_size)),
                    xf.transform_pos(pos2((i + 1) as f32 * self.cell_size, self.canvas_height())),
                ],
                if i % 10 == 0 { stroke_bold } else { stroke },
            );
        }
    }

    fn draw_world(&self, ui: &mut egui::Ui, xf: &RectTransform) {
        let font_id = TextStyle::Body.resolve(ui.style());
        let mut used_rect = Rect::NOTHING;

        for i in 0..10 {
            let text_rect = ui.painter().text(
                xf.transform_pos(pos2(
                    self.cell_size * (i + 1) as f32 + 4.0,
                    self.cell_size * (i + 2) as f32,
                )),
                Align2::LEFT_BOTTOM,
                "X",
                font_id.clone(),
                ui.visuals().text_color(),
            );
            used_rect = used_rect.union(text_rect);
            println!("used_rect is now {:?}", used_rect);
        }
        ui.allocate_rect(used_rect, Sense::hover());
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    loop {
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }

        /*
        clear_background(GRAY);
        let (rows, cols) = (15, 15);

        let (width, height) = (screen_width(), screen_height());
        let offset: f32 = 15.0;
        let (grid_width, grid_height) = (width - 2. * offset, height - 2. * offset);

        let (col_width, row_height): (f32, f32) =
            (grid_width / cols as f32, grid_height / rows as f32);

        set_camera(&Camera2D {
            zoom: macroquad::prelude::vec2(1., width / height),
            ..Default::default()
        });

        draw_line(0.3, 0.3, 1., 1., 0.01, RED);
        */

        let grid = Grid::new(2000, 2000);
        //        let (grid_width, grid_height) = (500u32, 500u32);
        egui_macroquad::ui(|ctx| {
            ctx.set_visuals(egui_macroquad::egui::Visuals::light());

            egui::TopBottomPanel::top("control_panel")
                .resizable(false)
                .min_height(32.)
                .show(ctx, |ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        ui.add(Label::new("FOOBAR"));
                        if ui.button("Button 1").clicked() {
                            println!("Button 1");
                        }
                        if ui.button("Button 2").clicked() {
                            println!("Button 2");
                        }
                    })
                });

            egui::CentralPanel::default().show(ctx, |ui| {
                ScrollArea::both()
                    .auto_shrink([false; 2])
                    .show_viewport(ui, |ui, viewport| {
                        ui.set_height(grid.canvas_height());
                        ui.set_width(grid.canvas_width());
                        let (response, painter) = ui.allocate_painter(
                            egui::Vec2::new(ui.available_width(), ui.available_height()),
                            egui::Sense::hover(),
                        );
                        println!("resp: {:?}", response.rect);
                        let to_screen = RectTransform::from_to(
                            Rect::from_min_size(viewport.left_top(), response.rect.size()),
                            response.rect,
                        );
                        // let to_screen = RectTransform::from_to(
                        //     response.rect.translate(viewport.left_top().to_vec2()),
                        //     response.rect,
                        // );
                        grid.draw_grid(&painter, &to_screen);
                        grid.draw_world(ui, &to_screen);
                    });
            });
        });

        egui_macroquad::draw();
        next_frame().await
    }
}
