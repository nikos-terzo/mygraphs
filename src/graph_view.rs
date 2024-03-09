use eframe::egui::{Color32, Rect, Response, Rounding, Sense, Shape, Stroke, Ui, Vec2, Widget};

pub const MAX_SIZE_4K: Vec2 = Vec2 { x: 4096., y: 2160. };
pub const MIN_GRAPH_VIEW_SIZE: Vec2 = Vec2 { x: 1280., y: 720. };

pub struct GraphView {
    pub size: Vec2,
    pub shapes: Vec<Shape>,
    // Inner translation of its view
    // pub viewport: Rect,
}

impl Widget for GraphView {
    fn ui(self, ui: &mut Ui) -> Response {
        let (clip_rect, resp) = ui.allocate_at_least(MIN_GRAPH_VIEW_SIZE, Sense::click_and_drag());
        let painter = ui.painter_at(clip_rect);
        self.shapes
            .into_iter()
            .filter(|shape| shape.visual_bounding_rect().intersects(clip_rect))
            .for_each(|shape| {
                // TODO: We lose ShapeIdx here
                painter.add(shape);
            });
        painter.rect(
            clip_rect,
            Rounding::ZERO,
            Color32::default(),
            Stroke {
                width: 2.,
                color: Color32::BLUE,
            },
        );
        resp
    }
}
