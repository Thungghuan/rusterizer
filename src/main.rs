#![allow(dead_code)]
mod mvp;
mod rasterizer;
mod triangle;

use mvp::*;
use nalgebra::{Point3, Vector3};
use opencv::{
    core::{self, Mat},
    highgui,
};

fn main() {
    const WIDTH: i32 = 700;
    const HEIGHT: i32 = 700;

    let mut rst = rasterizer::Rasterizer::new(WIDTH, HEIGHT);

    let tri = triangle::Triangle::new(
        Point3::new(200.0, 200.0, 2.0),
        Point3::new(350.0, 500.0, 2.0),
        Point3::new(500.0, 200.0, 2.0),
    );

    rst.clear();

    rst.set_view_matrix(get_view_matrix(Point3::new(0.0, 0.0, 5.0)));
    rst.set_model_matrix(get_model_matrix(Vector3::new(-1.0, 0.0, 0.0), 20.0));
    rst.set_projection_matrix(get_projection_matrix(45.0, 1.0, 0.1, 50.0));

    rst.draw(&[tri]);

    let image = unsafe {
        Mat::new_rows_cols_with_data(
            HEIGHT,
            WIDTH,
            core::CV_8UC3,
            rst.frame_buf.as_ptr() as *mut std::ffi::c_void,
            core::Mat_AUTO_STEP,
        )
        .unwrap()
    };

    highgui::imshow("Rusterizer", &image).unwrap();
    highgui::wait_key(0).unwrap();
}
