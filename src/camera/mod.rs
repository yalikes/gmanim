use nalgebra::Vector3;
pub struct Camera {
    pub position: Point3<GMFloat>,
    pub look_at: Vector3<GMFloat>,
    pub up_direction: Vector3<GMFloat>,
}
