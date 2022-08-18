use nalgebra::{Matrix4, Point2, Point3};

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
            // panic!("Error: Out of range");
            return;
        }

        let index = self.get_index(x, y);

        self.frame_buf[index] = color;
    }

    fn draw_line_helper(
        &mut self,
        begin: Point2<f32>,
        end: Point2<f32>,
        color: [u8; 3],
        is_flip: bool,
    ) {
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
            if is_flip {
                self.set_pixel(y, x, color);
            } else {
                self.set_pixel(x, y, color);
            }

            if d > 0 {
                y += yi;
                d -= 2 * dx;
            }

            d += 2 * dy;
        }
    }

    fn draw_line(&mut self, begin: Point2<f32>, end: Point2<f32>, color: [u8; 3]) {
        let x0 = begin.x;
        let y0 = begin.y;
        let x1 = end.x;
        let y1 = end.y;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        if dy < dx {
            if x0 < x1 {
                self.draw_line_helper(begin, end, color, false);
            } else {
                self.draw_line_helper(end, begin, color, false);
            }
        } else {
            if y0 < y1 {
                self.draw_line_helper(begin.yx(), end.yx(), color, true);
            } else {
                self.draw_line_helper(end.yx(), begin.yx(), color, true);
            }
        }
    }

    fn draw_triangle(&mut self, tri: &Triangle) {
        self.draw_line(tri.a().xy(), tri.b().xy(), tri.get_color());
        self.draw_line(tri.b().xy(), tri.c().xy(), tri.get_color());
        self.draw_line(tri.c().xy(), tri.a().xy(), tri.get_color());
    }

    pub fn draw(&mut self, triangle_list: &[Triangle]) {
        let mvp_m = self.projection_m * self.view_m * self.model_m;

        for tri in triangle_list {
            let mut triangle = tri.clone();

            let vertexs = triangle
                .get_vertex()
                .map(|vertex| mvp_m * vertex.to_homogeneous())
                .map(|mut vertex| {
                    vertex.x /= vertex.w;
                    vertex.y /= vertex.w;
                    vertex.z /= vertex.w;

                    vertex.x = 0.5 * (self.width as f32) * (vertex.x + 1.0);
                    vertex.y = 0.5 * (self.height as f32) * (1.0 - vertex.y);

                    let f1 = (50.0 - 0.1) / 2.0;
                    let f2 = (50.0 + 0.1) / 2.0;

                    vertex.z = vertex.z * f1 + f2;

                    vertex
                })
                .map(|vertex| Point3::from(vertex.xyz()));

            triangle.set_vertex(vertexs);

            self.draw_triangle(&triangle);
        }
    }
}
