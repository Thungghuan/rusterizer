#![allow(dead_code, unused)]
use opencv::{
    core::{self, Mat},
    highgui,
};

mod rasterizer;

fn model_matrix() {}

fn view_matrix() {}

fn projection_matrix() {}

fn main() {
    const WIDTH: i32 = 700;
    const HEIGHT: i32 = 700;

    let r = rasterizer::Rasterizer::new(WIDTH, HEIGHT);

    let image = unsafe {
        Mat::new_rows_cols_with_data(
            HEIGHT,
            WIDTH,
            core::CV_8UC3,
            r.frame_buf.as_ptr() as *mut std::ffi::c_void,
            core::Mat_AUTO_STEP,
        )
        .unwrap()
    };

    highgui::imshow("Rusterizer", &image);
    highgui::wait_key(0).unwrap();
}
