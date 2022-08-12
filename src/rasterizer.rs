use nalgebra::{Matrix4, Point2};

use crate::triangle::Triangle;

pub struct Rasterizer {
    width: i32,
    height: i32,

    model_m: Matrix4<f32>,
    view_m: Matrix4<f32>,
    projection_m: Matrix4<f32>,

    pub frame_buf: Vec<[u8; 3]>,
    depth_buf: Vec<f32>,
}

impl Rasterizer {
    pub fn new(width: i32, height: i32) -> Self {
        let length = (width * height) as usize;

        let frame_buf = vec![[0, 0, 0]; length];
        let depth_buf = vec![f32::INFINITY; length];

        Self {
            width,
            height,

            model_m: Matrix4::<f32>::default(),
            view_m: Matrix4::<f32>::default(),
            projection_m: Matrix4::<f32>::default(),

            frame_buf,
            depth_buf,
        }
    }

    pub fn clear(&mut self) {
        let length = (self.width * self.height) as usize;

        self.frame_buf = vec![[0, 0, 0]; length];
        self.depth_buf = vec![f32::INFINITY; length];
    }

    pub fn set_model_matrix(&mut self, model_matrix: Matrix4<f32>) {
        self.model_m = model_matrix;
    }

    pub fn set_view_matrix(&mut self, view_matrix: Matrix4<f32>) {
        self.view_m = view_matrix;
    }

    pub fn set_projection_matrix(&mut self, projection_matrix: Matrix4<f32>) {
        self.projection_m = projection_matrix;
    }

    pub fn get_index(&self, x: i32, y: i32) -> usize {
        // treat the left-bottom corner as origin
        ((self.height - y) * self.width + x) as usize
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: [u8; 3]) {
        if x < 0 || x > self.width || y < 0 || y > self.height {
            panic!("Error: Out of range");
        }

        let index = self.get_index(x, y);

        self.frame_buf[index] = color;
    }

    pub fn draw_line(&mut self, _begin: Point2<f32>, _end: Point2<f32>) {}

    pub fn draw(&mut self, triangle_list: &[Triangle]) {
        for triangle in triangle_list {
            println!("{}", triangle);
        }
    }
}
