use eframe::{egui::{Color32, Rect, Response, Rounding, Sense, Shape, Stroke, Ui, Vec2, Widget}, epaint::{CircleShape, Pos2}};

use crate::scale::{PaintCallbackNotSupported, TryScale};

pub const MAX_SIZE_4K: Vec2 = Vec2 { x: 4096., y: 2160. };
pub const MIN_GRAPH_VIEW_SIZE: Vec2 = Vec2 { x: 1280., y: 720. };

#[derive(Clone)]
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

/// TEMP
impl Default for GraphView {
    fn default() -> Self {
        Self {
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
        }
    }
}

impl TryScale for GraphView {
    type Error = PaintCallbackNotSupported;

    fn try_scale(&mut self, scale: f32) -> Result<(), Self::Error> {
        self.shapes
            .iter_mut()
            .try_for_each(|shape| shape.try_scale(scale))
    }
}
