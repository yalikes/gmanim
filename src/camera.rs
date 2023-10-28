use ndarray::Array1;
pub struct Camera<T> {
    pub position: Array1<T>,
    pub look_at: Array1<T>,
    pub up_direction: Array1<T>,
}