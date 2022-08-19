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

    let mut tri_1 = triangle::Triangle::new(
        Point3::new(2.0, 0.0, -2.0),
        Point3::new(0.0, 2.0, -2.0),
        Point3::new(-2.0, 0.0, -2.0),
    );

    let mut tri_2 = triangle::Triangle::new(
        Point3::new(3.5, -1.0, -5.0),
        Point3::new(2.5, 1.5, -5.0),
        Point3::new(-1.0, 0.5, -5.0),
    );

    tri_1.set_color([52, 134, 235]);
    tri_2.set_color([235, 205, 52]);

    let mut window = Window::new("Rusterizer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let cam_pos = Point3::new(0.0, 0.0, 5.0);
    let axis = Vector3::new(0.0, 0.0, 1.0);
    let angle = 0.0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        rst.clear();

        rst.set_view_matrix(get_view_matrix(cam_pos));
        rst.set_model_matrix(get_model_matrix(axis, angle));
        rst.set_projection_matrix(get_projection_matrix(45.0, 1.0, 0.1, 50.0));

        rst.draw(&[tri_1, tri_2]);

        let buffer = rst
            .frame_buf
            .iter()
            .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | (v[2] as u32))
            .collect::<Vec<u32>>();

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
