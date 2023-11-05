use std::sync::Arc;

use crate::Scene;
use nalgebra::{Matrix4, UnitVector3, Vector3, Vector4};
pub struct Camera<T> {
    pub position: Vector3<T>,
    pub look_at: UnitVector3<T>,
    pub up_direction: UnitVector3<T>,
}

impl<T> Camera<T> {
    pub fn render(&self, scene: &Scene) {}
}

struct Cube<T> {
    points: Vec<Vector3<T>>,
}

#[test]
fn test_take_a_shot() {
    let camera = Camera::<f32> {
        position: Vector3::new(1.0, 1.0, -1.0),
        look_at: UnitVector3::new_normalize(Vector3::new(-1.0, -1.0, 1.0)),
        up_direction: UnitVector3::new_normalize(Vector3::new(-0.40824829, 0.81649658, 0.40824829)),
    };
    let p0 = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let T = Matrix4::new(
        1.0,
        0.0,
        0.0,
        -camera.position[0],
        0.0,
        1.0,
        0.0,
        -camera.position[1],
        0.0,
        0.0,
        1.0,
        -camera.position[2],
        0.0,
        0.0,
        0.0,
        1.0,
    );
    let x_axis_after = camera.up_direction.cross(&camera.look_at);
    let rotate_rev = Matrix4::from_columns(&[
        x_axis_after.to_homogeneous(),
        camera.up_direction.to_homogeneous(),
        camera.look_at.to_homogeneous(),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    ]);
    let rotate = rotate_rev.transpose();
    let trans_matrix = rotate * T;
    println!("{:?}", trans_matrix * p0);
}
