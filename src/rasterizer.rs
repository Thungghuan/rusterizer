use nalgebra::Vector3;

pub struct Rasterizer {
    width: i32,
    height: i32,

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
            frame_buf,
            depth_buf,
        }
    }
}
