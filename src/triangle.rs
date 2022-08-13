use std::fmt::Display;

use nalgebra::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    vertex: [Point3<f32>; 3],
    color: [u8; 3],
}

impl Triangle {
    pub fn new(a: Point3<f32>, b: Point3<f32>, c: Point3<f32>) -> Self {
        let vertex = [a, b, c];

        Self {
            vertex,
            color: [255, 255, 255],
        }
    }

    pub fn set_vertex(&mut self, vertex: [Point3<f32>; 3]) {
        self.vertex = vertex;
    }

    pub fn get_vertex(&self) -> [Point3<f32>; 3] {
        self.vertex
    }

    pub fn a(&self) -> Point3<f32> {
        self.vertex[0]
    }

    pub fn b(&self) -> Point3<f32> {
        self.vertex[1]
    }

    pub fn c(&self) -> Point3<f32> {
        self.vertex[2]
    }

    pub fn set_color(&mut self, color: [u8; 3]) {
        self.color = color;
    }

    pub fn get_color(&self) -> [u8; 3] {
        self.color
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Triangle:\n  A: {},\n  B: {},\n  C: {},\n  color: {:?}\n",
            self.a(),
            self.b(),
            self.c(),
            self.color
        )
    }
}
