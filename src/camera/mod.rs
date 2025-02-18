use crate::math_utils::constants::PI;
use crate::GMFloat;
use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3, Vector4};
use usvg::tiny_skia_path::Scalar;

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<GMFloat>,
    look_at: Vector3<GMFloat>, // attention that this vector is assumed to be a unit vector
    up_direction: Vector3<GMFloat>,
    projection: Projection,
}

#[derive(Debug)]
pub enum Projection {
    Perspective(PerspectiveSetting),
    Orthographic(OrthographicSetting),
}

impl Default for Projection {
    fn default() -> Self {
        Projection::Perspective(PerspectiveSetting::default())
    }
}

#[derive(Debug)]
pub struct PerspectiveSetting {
    near: GMFloat,
    far: GMFloat,
    fovy: GMFloat,
    aspect: GMFloat,
}

impl Default for PerspectiveSetting {
    fn default() -> Self {
        Self {
            near: 1.0,
            far: 2.0,
            fovy: PI / 2,
            aspect: 16.0 / 9.0,
        }
    }
}

impl PerspectiveSetting {
    pub fn get_perspective_project_matrix(&self) -> Matrix4<GMFloat> {
        Perspective3::new(self.aspect, self.fovy, self.near, self.far)
            .as_matrix()
            .to_owned()
    }
}

#[derive(Debug)]
pub struct OrthographicSetting {
    left: GMFloat,
    right: GMFloat,
    bottom: GMFloat,
    top: GMFloat,
    near: GMFloat,
    far: GMFloat,
}

impl OrthographicSetting {
    pub fn new(
        left: GMFloat,
        right: GMFloat,
        bottom: GMFloat,
        top: GMFloat,
        near: GMFloat,
        far: GMFloat,
    ) -> Self {
        Self {
            left,
            right,
            bottom,
            top,
            near,
            far,
        }
    }
    pub fn get_orthographic_project_matrix(&self) -> Matrix4<GMFloat> {
        Matrix4::from_columns(&[
            Vector4::new(2.0 / (self.right - self.left), 0.0, 0.0, 0.0),
            Vector4::new(0.0, 2.0 / (self.top - self.bottom), 0.0, 0.0),
            Vector4::new(0.0, 0.0, 2.0 / (self.near - self.far), 0.0),
            Vector4::new(
                -(self.right + self.left) / (self.right - self.left),
                -(self.top + self.bottom) / (self.top - self.bottom),
                -(self.near + self.far) / (self.near - self.far),
                1.0,
            ),
        ])
    }
}

impl Default for OrthographicSetting {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: -2.0,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point3::origin(),
            -Vector3::z(),
            Vector3::y(),
            Projection::default(),
        )
    }
}

impl Camera {
    pub fn new(
        position: Point3<GMFloat>,
        look_at: Vector3<GMFloat>,
        up_direction: Vector3<GMFloat>,
        projection: Projection,
    ) -> Self {
        Self {
            position,
            look_at: look_at.normalize(),
            up_direction: up_direction.normalize(),
            projection,
        }
    }
    pub fn set_look_at(&mut self, look_at: Vector3<GMFloat>) {
        self.look_at = look_at.normalize();
    }
    pub fn set_up_direction(&mut self, up_direction: Vector3<GMFloat>) {
        self.up_direction = up_direction.normalize();
    }
    pub fn get_camera_transform_matrix(&self) -> Matrix4<GMFloat> {
        Isometry3::look_at_rh(
            &self.position,
            &(self.position + self.look_at),
            &self.up_direction,
        )
        .to_homogeneous()
    }
    pub fn get_projection_transform_matrix(&self) -> Matrix4<GMFloat> {
        match &self.projection {
            Projection::Perspective(p) => p.get_perspective_project_matrix(),
            Projection::Orthographic(o) => o.get_orthographic_project_matrix(),
        }
    }
}
#[test]
pub fn test_camera_transform() {
    let mut r = Camera::default();
    r.position = Point3::new(0.5, 0.5, -0.5);
    r.set_up_direction(Vector3::new(1.0, 1.0, 0.0));
    let c = r.get_camera_transform_matrix();
    let p1 = Point3::new(1.0, 0.0, 0.0).to_homogeneous();
    let p2 = Point3::new(0.0, 1.0, 0.0).to_homogeneous();
    let p3 = Point3::new(0.0, 0.0, 1.0).to_homogeneous();

    assert!(
        ((c * p1) - Point3::new((2.0 as GMFloat).sqrt() / 2.0, 0.0, 0.5).to_homogeneous()).norm()
            < 1e-3
    );
    assert!(
        ((c * p2) - Point3::new(-(2.0 as GMFloat).sqrt() / 2.0, 0.0, 0.5).to_homogeneous()).norm()
            < 1e-3
    );
    assert!(
        ((c * p3) - Point3::new(0.0, -(2.0 as GMFloat).sqrt() / 2.0, 1.5).to_homogeneous()).norm()
            < 1e-3
    );
}

// we may need to write 3d render ourself, e.g. cannot use tinyskia, since we need to handle z-buffer.

// pipe line
// camera transform -> perspective transform -> viewport transform
