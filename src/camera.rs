use nalgebra::Vector3;
pub struct Camera<T> {
    pub position: Vector3<T>,
    pub look_at: Vector3<T>,
    pub up_direction: Vector3<T>,
}