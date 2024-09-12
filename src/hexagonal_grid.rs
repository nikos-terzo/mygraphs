use eframe::egui::Pos2;


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

