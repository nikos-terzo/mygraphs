pub mod graph_view;
pub mod scale;

use eframe::egui;
use eframe::egui::epaint::{CircleShape, Color32, Pos2, Shape, Stroke, Vec2};
use graph_view::GraphView;

#[derive(Default)]
pub struct MyEguiApp {
    // pub graph_view_viewport: Option<Rect>,
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let heading = ui.heading("Hello World!");

            if heading.contains_pointer() {
                let maybe_scroll = ctx.input(|i| i.zoom_delta());
                println!("Scroll {:?}", maybe_scroll);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // let viewport = match self.graph_view_viewport {
            //     Some(v) => v,
            //     None => {
            //         let min = ui.next_widget_position();
            //         Rect {
            //             min,
            //             max: min + GRAPH_VIEW_SIZE,
            //         }
            //     }
            // };
            let graph_view = ui.add(GraphView {
                size: Vec2 { x: 500., y: 500. },
                shapes: vec![
                    Shape::Circle(CircleShape {
                        center: Pos2 { x: 0., y: 0. },
                        radius: 200.,
                        fill: Color32::WHITE,
                        stroke: Stroke {
                            width: 10.,
                            color: Color32::RED,
                        },
                    }),
                    Shape::Circle(CircleShape {
                        center: Pos2 { x: 110., y: 110. },
                        radius: 40.,
                        fill: Color32::BLUE,
                        stroke: Stroke {
                            width: 5.,
                            color: Color32::RED,
                        },
                    }),
                ],
                // viewport,
            });

            if graph_view.contains_pointer() {
                let maybe_scroll = ctx.input(|i| i.zoom_delta());
                println!("Scroll {:?}", maybe_scroll);
            }
        });
    }
}
