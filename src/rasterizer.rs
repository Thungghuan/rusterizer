use nalgebra::Matrix4;

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

    pub fn set_model_matrix(&mut self, model_matrix: Matrix4<f32>) {
        self.model_m = model_matrix;
    }

    pub fn set_view_matrix(&mut self, view_matrix: Matrix4<f32>) {
        self.view_m = view_matrix;
    }

    pub fn set_projection_matrix(&mut self, projection_matrix: Matrix4<f32>) {
        self.projection_m = projection_matrix;
    }
}
