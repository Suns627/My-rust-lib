// 加法函数
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
//减法函数
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
// 乘法函数
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
// 除法函数
pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Cannot divide by zero")
     } else {
        Ok(a / b)
    }
}

use std::f64::consts::PI;
pub fn calculate_polygon_vertices(num_sides: usize, max_radius: f64, center_x: f64, center_y: f64) -> Vec<(f64, f64)> {
    let side_length = 2.0 * max_radius * (PI / num_sides as f64).sin();
    let mut vertices: Vec<(f64, f64)> = Vec::new();
    for i in 0..num_sides {
        let angle = 2.0 * PI * (i as f64) / (num_sides as f64);
        let x = center_x + max_radius * angle.cos();
        let y = center_y + max_radius * angle.sin();
        vertices.push((x, y));
    }
    vertices
}

