use nalgebra::{Matrix3, Matrix4, Point3, Vector3};
use std::f32::consts::PI;

fn get_radius(degree: f32) -> f32 {
    degree * PI / 180.0
}

#[rustfmt::skip]
pub fn get_view_matrix(eye_pos: Point3<f32>) -> Matrix4<f32> {
    Matrix4::new(
        1.0, 0.0, 0.0, -eye_pos.x,
        0.0, 1.0, 0.0, -eye_pos.y,
        0.0, 0.0, 1.0, -eye_pos.z,
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn get_model_matrix(rotate_axis: Vector3<f32>, rotate_angle: f32) -> Matrix4<f32> {
    let rotate_radius = get_radius(rotate_angle);

    #[rustfmt::skip]
    let axis_cross_m = Matrix3::new(
        0.0, -rotate_axis.z, rotate_axis.y,
        rotate_axis.z, 0.0, rotate_axis.x,
        -rotate_axis.y, rotate_axis.x, 0.0,
    );

    let rotation_m = f32::cos(rotate_radius) * Matrix3::identity()
        + (1.0 - f32::cos(rotate_radius)) * rotate_axis * rotate_axis.transpose()
        + f32::sin(rotate_radius) * axis_cross_m;

    rotation_m.to_homogeneous()
}

pub fn get_projection_matrix(fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Matrix4<f32> {
    // nagate z_near & z_far
    let z_near = -z_near;
    let z_far = -z_far;

    // find the bounding box
    let half = get_radius(fov / 2.0);
    let top = half.tan() * z_near.abs();
    let bottom = -top;
    let right = -top * aspect_ratio;
    let left = -right;

    // translate to origin
    #[rustfmt::skip]
    let translate_m = Matrix4::new(
        1.0, 0.0, 0.0, -(right + left) / 2.0,
        0.0, 1.0, 0.0, -(top + bottom) / 2.0,
        0.0, 0.0, 1.0, -(z_near + z_far) / 2.0,
        0.0, 0.0, 0.0, 1.0,
    );

    // scale length to 1
    #[rustfmt::skip]
    let scale_m = Matrix4::new(
        2.0 / (right - left), 0.0, 0.0, 0.0,
        0.0, 2.0 / (top - bottom), 0.0, 0.0,
        0.0, 0.0, 2.0 / (z_near - z_far), 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    // perspective projection
    #[rustfmt::skip]
    let perspect_m = Matrix4::new(
        z_near, 0.0, 0.0, 0.0,
        0.0, z_near, 0.0, 0.0,
        0.0, 0.0, z_near + z_far, -z_near * z_far,
        0.0, 0.0, 1.0, 0.0
    );

    scale_m * translate_m * perspect_m
}
