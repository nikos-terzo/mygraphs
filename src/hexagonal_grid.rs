use eframe::egui::Pos2;
use voronoi::Point;


const SIN60: f32 = 0.86602540378443864676372317075293618347140262690519031402790348972596650845440001854057309337862428783781307070770335151498497254749947623940582775604718682426404661595115279103398741005054233746163251;
const UR: [f32; 2] = [0.5, SIN60];
const DR: [f32; 2] = [0.5, -SIN60];
const A: f32 = 100_f32;

pub fn generate_horizontal_line0(c_index: usize, x_index: usize) -> [Pos2; 2] {
    let y = (c_index as f32) * A * 3_f32.sqrt();
    let x0 = (x_index as f32) * 3. * A;
    let x1 = x0 + A;
    [Pos2 { x: x0, y }, Pos2 { x: x1, y }]
}

pub fn generate_horizontal_line1(c_index: usize, x_index: usize) -> [Pos2; 2] {
    let y = A * 3_f32.sqrt() / 2. + (c_index as f32) * A * 3_f32.sqrt();
    let x0 = 1.5 * A + (x_index as f32) * 3. * A;
    let x1 = x0 + A;
    [Pos2 { x: x0, y }, Pos2 { x: x1, y }]
}

pub fn generate_ur_line(c_index: i64, x_index: i64) -> [Pos2; 2] {
    let c = (c_index as f32) * A * 3_f32.sqrt();
    let t0 = -A + (x_index as f32) * (3. * A);
    let t1 = t0 + A;
    let x0 = t0 * UR[0];
    let y0 = t0 * UR[1] + c;
    let x1 = t1 * UR[0];
    let y1 = t1 * UR[1] + c;
    [Pos2 { x: x0, y: y0 }, Pos2 { x: x1, y: y1 }]
}

pub fn generate_dr_line(c_index: i64, x_index: i64) -> [Pos2; 2] {
    let c = (c_index as f32) * A * 3_f32.sqrt();
    let t0 = -A + (x_index as f32) * (3. * A);
    let t1 = t0 + A;
    let x0 = t0 * DR[0];
    let y0 = t0 * DR[1] + c;
    let x1 = t1 * DR[0];
    let y1 = t1 * DR[1] + c;
    [Pos2 { x: x0, y: y0 }, Pos2 { x: x1, y: y1 }]
}

const N_POINTS: usize = 10;

pub fn surround(line_segment: &[Pos2; 2], n_points: Option<usize>) -> Vec<Point> {
    let n_points = n_points.unwrap_or(N_POINTS) + 1;

    let dir = line_segment[1] - line_segment[0];
    let vert = Pos2 { x: -dir.y, y: dir.x };
    let length = (dir.x.powi(2) + dir.y.powi(2)).sqrt()/10.;
    let vert = vert / length;

    let mut points = Vec::with_capacity(n_points*2);
    for i in 1..n_points {
        let t = i as f32 / (n_points as f32);
        let point = line_segment[0] + t * dir;
        let point0 = point + vert.to_vec2();
        let point1 = point - vert;
        points.push(Point { x: (point0.x as f64).into(), y: (point0.y as f64).into() });
        points.push(Point { x: (point1.x as f64).into(), y: (point1.y as f64).into() });
    }
    points
}
