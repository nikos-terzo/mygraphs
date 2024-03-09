use std::sync::Arc;

use eframe::epaint::{
    CircleShape, CubicBezierShape, Galley, Mesh, PathShape, QuadraticBezierShape, RectShape, Shape,
    TextShape,
};

pub trait Scale {
    fn scale(&mut self, scale: f32);
}

pub trait TryScale {
    type Error;
    fn try_scale(&mut self, scale: f32) -> Result<(), Self::Error>;
}

#[derive(thiserror::Error, Debug)]
#[error("Shape::Callback(PaintCallback) not supported")]
pub struct PaintCallbackNotSupported;

impl TryScale for Shape {
    type Error = PaintCallbackNotSupported;

    fn try_scale(&mut self, scale: f32) -> Result<(), Self::Error> {
        match self {
            Shape::Noop => {}
            Shape::Vec(shapes) => {
                shapes
                    .iter_mut()
                    .map(|shape| shape.try_scale(scale))
                    .collect::<Result<Vec<_>, _>>()?;
            }
            Shape::Circle(CircleShape {
                center,
                radius,
                stroke,
                ..
            }) => {
                center.x *= scale;
                center.y *= scale;
                *radius *= scale;
                stroke.width *= scale;
            }
            Shape::LineSegment {
                points: [start, end],
                stroke,
            } => {
                start.x *= scale;
                start.y *= scale;
                end.x *= scale;
                end.y *= scale;
                stroke.width *= scale;
            }
            Shape::Path(PathShape { points, stroke, .. }) => {
                points.iter_mut().for_each(|point| {
                    point.x *= scale;
                    point.y *= scale;
                });
                stroke.width *= scale;
            }
            Shape::Rect(RectShape { rect, stroke, .. }) => {
                rect.min.x *= scale;
                rect.min.y *= scale;
                rect.max.x *= scale;
                rect.max.y *= scale;
                stroke.width *= scale;
            }
            Shape::Text(TextShape { pos, galley, .. }) => {
                pos.x *= scale;
                pos.y *= scale;
                // This might be slow
                let mut new_galley = Galley::from(galley.as_ref().clone());
                new_galley.pixels_per_point *= scale;
                *galley = Arc::new(new_galley);
                todo!("Perf test?");
            }
            Shape::Mesh(Mesh { vertices, .. }) => {
                vertices.iter_mut().for_each(|vertex| {
                    vertex.pos.x *= scale;
                    vertex.pos.y *= scale;
                });
            }
            Shape::QuadraticBezier(QuadraticBezierShape { points, stroke, .. }) => {
                points.iter_mut().for_each(|point| {
                    point.x *= scale;
                    point.y *= scale;
                });
                stroke.width *= scale;
            }
            Shape::CubicBezier(CubicBezierShape { points, stroke, .. }) => {
                points.iter_mut().for_each(|point| {
                    point.x *= scale;
                    point.y *= scale;
                });
                stroke.width *= scale;
            }
            Shape::Callback(_) => {
                return Err(PaintCallbackNotSupported);
            }
        }
        Ok(())
    }
}
