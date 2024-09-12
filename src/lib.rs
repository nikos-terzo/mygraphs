pub mod graph_view;
pub mod scale;
pub mod hexagonal_grid;

use eframe::egui;

use graph_view::GraphView;
use scale::TryScale;


#[derive(Default)]
pub struct MyEguiApp {
    // pub graph_view_viewport: Option<Rect>,
    pub graph_view: GraphView,
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
            let graph_view = ui.add(self.graph_view.clone());

            if graph_view.contains_pointer() {
                let pinch_neg = ctx.input(|i| i.zoom_delta());
                println!("pinch {:?}", pinch_neg);
                let scroll = ctx.input(|i| i.raw_scroll_delta);
                println!("Scroll {:?}", scroll);

                let Some(mouse_pos) = graph_view.hover_pos() else {
                    println!("No mouse pos");
                    return;
                };
                // TODO: Compare floats (epsilon)
                if pinch_neg != 1.0 {
                    self.graph_view.try_scale(pinch_neg).unwrap_or_else(|_| {
                        println!("Failed to scale");
                    });

                    let translate = mouse_pos - mouse_pos * pinch_neg;
                    self.graph_view.shapes.iter_mut().for_each(|shape| {
                        shape.translate(translate);
                    });
                } else if scroll.x != 0.0 {
                    self.graph_view
                        .try_scale(0.01 * scroll.x + 1.)
                        .unwrap_or_else(|_| {
                            println!("Failed to scale");
                        });
                }
            }
        });
    }
}
