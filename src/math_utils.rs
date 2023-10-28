use ndarray::{arr2, Array2};
pub fn three_d_transform_matrix<T: ndarray::NdFloat + std::convert::From<f64>>(
    axis: &ndarray::Array1<T>,
    theta: &T,
) -> Array2<T> {
    let ux = axis[[0]];
    let uy = axis[[1]];
    let uz = axis[[2]];
    let one = Into::<T>::into(1_f64);
    arr2(&[
        [
            theta.cos() + ux * ux * (one - theta.cos()),
            ux * uy * (one - theta.cos()) - uz * theta.sin(),
            ux * uz * (one - theta.cos()) + uy * theta.sin(),
        ],
        [
            uy * ux * (one - theta.cos()) + uz * theta.sin(),
            theta.cos() + uy * uy * (one - theta.cos()),
            uy * uz * (one - theta.cos()) - ux * theta.sin(),
        ],
        [
            uz * ux * (one - theta.cos()) - uy * theta.sin(),
            uz * uy * (one - theta.cos()) + ux * theta.sin(),
            theta.cos() + uz * uz * (one - theta.cos()),
        ],
    ])
}

#[test]
fn test_simple_rotate() {
    let axis = ndarray::arr1(&[1.0,1.0,1.0]) / 3_f32.sqrt();
    let theta = std::f32::consts::PI * 2_f32 / 3_f32;
    let v = ndarray::arr1(&[1.0,1.0,1.0]);
    use ndarray_linalg::norm::Norm;
    
    // assert!(
    // );
}
