use std::fmt::Display;

use nalgebra::{Matrix3, Point2, Point3, Point4, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    vertex: [Point4<f32>; 3],
    color: [u8; 3],
}

impl Triangle {
    pub fn new(a: Point3<f32>, b: Point3<f32>, c: Point3<f32>) -> Self {
        let vertex = [a, b, c].map(|p| Point4::new(p.x, p.y, p.z, 0.0));

        Self {
            vertex,
            color: [255, 255, 255],
        }
    }

    pub fn set_vertex(&mut self, vertex: [Point4<f32>; 3]) {
        self.vertex = vertex;
    }

    pub fn get_vertex(&self) -> [Point4<f32>; 3] {
        self.vertex
    }

    pub fn a(&self) -> Point4<f32> {
        self.vertex[0]
    }

    pub fn b(&self) -> Point4<f32> {
        self.vertex[1]
    }

    pub fn c(&self) -> Point4<f32> {
        self.vertex[2]
    }

    pub fn set_color(&mut self, color: [u8; 3]) {
        self.color = color;
    }

    pub fn get_color(&self) -> [u8; 3] {
        self.color
    }

    pub fn get_bounding(&self) -> [f32; 4] {
        [
            |v: Point4<f32>| v.x, // left
            |v: Point4<f32>| v.x, // right
            |v: Point4<f32>| v.y, // top
            |v: Point4<f32>| v.y, // bottom
        ]
        .iter()
        .zip([f32::min, f32::max, f32::max, f32::min])
        .map(|(get_coord, reducer)| {
            self.get_vertex()
                .map(get_coord)
                .into_iter()
                .reduce(reducer)
                .unwrap()
        })
        .collect::<Vec<f32>>()
        .try_into()
        .unwrap()
    }

    pub fn include_point(&self, point: Point2<u32>) -> bool {
        let p = Point3::new(point.x as f32, point.y as f32, 1.0);

        let ab_v = (self.b() - self.a()).xyz();
        let bc_v = (self.c() - self.b()).xyz();
        let ca_v = (self.a() - self.c()).xyz();

        let ap_v = p - self.a().xyz();
        let bp_v = p - self.b().xyz();
        let cp_v = p - self.c().xyz();

        let ab_z = ab_v.cross(&ap_v).z;
        let bc_z = bc_v.cross(&bp_v).z;
        let ca_z = ca_v.cross(&cp_v).z;

        (ab_z > 0.0 && bc_z > 0.0 && ca_z > 0.0) || (ab_z < 0.0 && bc_z < 0.0 && ca_z < 0.0)
    }

    // Compute Barycentric Coordinates on triangles
    // Refer to: https://en.wikipedia.org/wiki/Barycentric_coordinate_system
    pub fn get_bary_centric(&self, point: Point2<u32>) -> [f32; 3] {
        let r_m = Matrix3::from_columns(&[
            Vector3::new(1.0, self.a().x, self.a().y),
            Vector3::new(1.0, self.b().x, self.b().y),
            Vector3::new(1.0, self.c().x, self.c().y),
        ]);

        let p_v = Vector3::new(1.0, point.x as f32, point.y as f32);
        let lambda_v = r_m.try_inverse().unwrap() * p_v;

        [lambda_v.x, lambda_v.y, lambda_v.z]
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
