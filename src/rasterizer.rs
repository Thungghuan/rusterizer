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
        // treat the top-left corner as origin
        (y * self.width + x) as usize
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: [u8; 3]) {
        if x < 0 || x > self.width || y < 0 || y > self.height {
            panic!("Error: Out of range");
        }

        let index = self.get_index(x, y);

        self.frame_buf[index] = color;
    }

    fn draw_line_low(&mut self, begin: Point2<f32>, end: Point2<f32>, color: [u8; 3]) {
        let x0 = begin.x as i32;
        let y0 = begin.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;
        let dx = x1 - x0;
        let mut dy = y1 - y0;
        let mut yi = 1;

        if dy < 0 {
            yi = -1;
            dy = -dy;
        }

        let mut d = 2 * dy - dx;
        let mut y = y0;

        for x in x0..=x1 {
            self.set_pixel(x, y, color);

            if d > 0 {
                y += yi;
                d -= 2 * dx;
            }

            d += 2 * dy;
        }
    }

    fn draw_line_high(&mut self, begin: Point2<f32>, end: Point2<f32>, color: [u8; 3]) {
        let x0 = begin.x as i32;
        let y0 = begin.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;
        let mut dx = x1 - x0;
        let dy = y1 - y0;
        let mut xi = 1;

        if dx < 0 {
            xi = -1;
            dx = -dx;
        }

        let mut d = 2 * dx - dy;
        let mut x = x0;

        for y in y0..=y1 {
            self.set_pixel(x, y, color);

            if d > 0 {
                x += xi;
                d -= 2 * dy;
            }

            d += 2 * dx;
        }
    }

    pub fn draw_line(&mut self, begin: Point2<f32>, end: Point2<f32>, color: [u8; 3]) {
        let x0 = begin.x;
        let y0 = begin.y;
        let x1 = end.x;
        let y1 = end.y;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        if dy < dx {
            if x0 < x1 {
                self.draw_line_low(begin, end, color);
            } else {
                self.draw_line_low(end, begin, color);
            }
        } else {
            if y0 < y1 {
                self.draw_line_high(begin, end, color);
            } else {
                self.draw_line_high(end, begin, color);
            }
        }
    }

    pub fn draw(&mut self, triangle_list: &[Triangle]) {
        for triangle in triangle_list {
            self.draw_line(triangle.a().xy(), triangle.b().xy(), triangle.get_color());
            self.draw_line(triangle.b().xy(), triangle.c().xy(), triangle.get_color());
            self.draw_line(triangle.c().xy(), triangle.a().xy(), triangle.get_color());
        }
    }
}
