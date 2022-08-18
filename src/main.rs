#![allow(dead_code)]
mod mvp;
mod rasterizer;
mod triangle;

use minifb::{Key, Window, WindowOptions};
use mvp::*;
use nalgebra::{Point3, Vector3};

fn main() {
    const WIDTH: usize = 700;
    const HEIGHT: usize = 700;

    let mut rst = rasterizer::Rasterizer::new(WIDTH, HEIGHT);

    let tri = triangle::Triangle::new(
        Point3::new(2.0, 0.0, -2.0),
        Point3::new(0.0, 2.0, -2.0),
        Point3::new(-2.0, 0.0, -2.0),
    );

    rst.clear();

    rst.set_view_matrix(get_view_matrix(Point3::new(0.0, 0.0, 5.0)));
    rst.set_model_matrix(get_model_matrix(Vector3::new(-1.0, 0.0, 0.0), 0.0));
    rst.set_projection_matrix(get_projection_matrix(45.0, 1.0, 0.1, 50.0));

    rst.draw(&[tri]);

    let buffer = rst
        .frame_buf
        .iter()
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | (v[2] as u32))
        .collect::<Vec<u32>>();

    let mut window = Window::new("Rusterizer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
