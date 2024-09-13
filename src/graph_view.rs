use eframe::{
    egui::{Color32, Response, Rounding, Sense, Shape, Stroke, Ui, Vec2, Widget},
    epaint::{CircleShape, Pos2},
};
use rand::Rng;
use voronoi::Point;

use crate::{hexagonal_grid::surround, scale::{PaintCallbackNotSupported, TryScale}};

use crate::hexagonal_grid::{
    generate_dr_line, generate_horizontal_line0, generate_horizontal_line1, generate_ur_line,
};


pub const MAX_SIZE_4K: Vec2 = Vec2 { x: 4096., y: 2160. };
pub const DEFAULT_GRAPH_VIEW_SIZE: Vec2 = Vec2 { x: 1920., y: 1080. };
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
        let (clip_rect, resp) = ui.allocate_at_least(DEFAULT_GRAPH_VIEW_SIZE, Sense::click_and_drag());
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
        let mut shapes = vec![];

        // Create random 200 points
        // for _ in 0..2000 {
        //     let x = rng.gen_range(0_f32..2000_f32);
        //     let y = rng.gen_range(0_f32..2000_f32);
        //     shapes.push(Shape::Circle(CircleShape {
        //         center: Pos2 { x, y },
        //         radius: 10.,
        //         fill: Color32::RED,
        //         stroke: Stroke {
        //             width: 2.,
        //             color: Color32::BLUE,
        //         },
        //     }));
        //     points.push(Point { x: (x as f64).into(), y: (y as f64).into() });
        // }


        let mut voronoi_centers = vec![];
        // Create hexagonal grid
        for c_index in 0..400 {
            for x_index in 0..20 {
                let line_pos = generate_horizontal_line0(c_index, x_index);
                voronoi_centers.append(&mut surround(&line_pos, None).to_vec());
                shapes.push(Shape::LineSegment {
                    points: line_pos,
                    stroke: Stroke {
                        width: 2.,
                        color: Color32::BLUE,
                    },
                });
                let line_pos = generate_horizontal_line1(c_index, x_index);
                voronoi_centers.append(&mut surround(&line_pos, None).to_vec());
                shapes.push(Shape::LineSegment {
                    points: line_pos,
                    stroke: Stroke {
                        width: 2.,
                        color: Color32::RED,
                    },
                });
                let ur_line = generate_ur_line(c_index as i64 - 200, x_index as i64);
                voronoi_centers.append(&mut surround(&ur_line, None).to_vec());
                shapes.push(Shape::LineSegment {
                    points: ur_line,
                    stroke: Stroke {
                        width: 2.,
                        color: Color32::GREEN,
                    },
                });
                let dr_line = generate_dr_line(c_index as i64 - 200, x_index as i64);
                voronoi_centers.append(&mut surround(&dr_line, None).to_vec());
                shapes.push(Shape::LineSegment {
                    points: dr_line,
                    stroke: Stroke {
                        width: 2.,
                        color: Color32::YELLOW,
                    },
                });
            }
        }

        let dcel = voronoi::voronoi(voronoi_centers, 2000.);

        let mut rng = rand::thread_rng();
        let polygons = voronoi::make_polygons(&dcel);
        for polygon in polygons {
            let corners = polygon.iter().map(|point| to_pos2(*point)).collect::<Vec<Pos2>>();
            let color = Color32::from_rgb(rng.gen(), rng.gen(), rng.gen());
            shapes.push(Shape::convex_polygon(corners, color, Stroke::default()));
        };
        
        Self {
            size: Vec2 { x: 800., y: 800. },
            shapes,
        }
    }
}

pub fn to_pos2(point: Point) -> Pos2 {
    let x: f64 = point.x.into();
    let y: f64 = point.y.into();
    Pos2 { x: x as f32, y: y as f32 }
}

impl TryScale for GraphView {
    type Error = PaintCallbackNotSupported;

    fn try_scale(&mut self, scale: f32) -> Result<(), Self::Error> {
        self.shapes
            .iter_mut()
            .try_for_each(|shape| shape.try_scale(scale))
    }
}
