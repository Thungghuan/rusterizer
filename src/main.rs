#![allow(dead_code)]
mod mvp;
mod rasterizer;

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

    rst.set_view_matrix(get_view_matrix(Point3::new(0.0, 0.0, 5.0)));
    rst.set_model_matrix(get_model_matrix(Vector3::new(-1.0, 0.0, 0.0), 20.0));
    rst.set_projection_matrix(get_projection_matrix(45.0, 1.0, 0.1, 50.0));

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
