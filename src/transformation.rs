use super::matrix::Matrix;

pub enum Axis {
    X,
    Y,
    Z,
}

pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
    let mut matrix = Matrix::identity(4);
    matrix[0][3] = x;
    matrix[1][3] = y;
    matrix[2][3] = z;

    matrix
} 

pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
    let mut matrix = Matrix::identity(4);
    matrix[0][0] = x;
    matrix[1][1] = y;
    matrix[2][2] = z;

    matrix
}

pub fn rotate(radians: f64, axis: Axis) -> Matrix {
    let mut matrix = Matrix::identity(4);

    match axis {
        Axis::X => {
            matrix[1][1] = radians.cos();
            matrix[1][2] = -radians.sin();
            matrix[2][1] = radians.sin();
            matrix[2][2] = radians.cos();
        },
        Axis::Y => {
            matrix[0][0] = radians.cos();
            matrix[0][2] = radians.sin();
            matrix[2][0] = -radians.sin();
            matrix[2][2] = radians.cos();
        },
        Axis::Z => {
            matrix[0][0] = radians.cos();
            matrix[0][1] = -radians.sin();
            matrix[1][0] = radians.sin();
            matrix[1][1] = radians.cos();
        }
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::tuple::Tuple;
    use std::f64::consts::PI;

    #[test]
    fn multiplying_by_translation_matrix() {
        let transform = translate(5., -3., 2.);
        let point = Tuple::point(-3., 4., 5.);

        let expected = Tuple::point(2., 1., 7.);

        let actual = transform * point;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_by_inverse_of_translation_matrix() {
        let transform = translate(5., -3., 2.);
        let inverse = transform.inverse().unwrap();
        let point = Tuple::point(-3., 4., 5.);

        let expected = Tuple::point(-8., 7., 3.);

        let actual = inverse * point;

        assert_eq!(expected, actual);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translate(5., -3., 2.);
        let vector = Tuple::vector(-3., 4., 5.);

        let expected = vector;

        let actual = transform * vector;

        assert_eq!(expected, actual);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = scale(2., 3., 4.);
        let point = Tuple::point(-4., 6., 8.);

        let expected = Tuple::point(-8., 18., 32.);

        let actual = transform * point;

        assert_eq!(expected, actual);
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = scale(2., 3., 4.);
        let vector = Tuple::vector(-4., 6., 8.);

        let expected = Tuple::vector(-8., 18., 32.);

        let actual = transform * vector;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_by_inverse_of_scaling_matrix() {
        let transform = scale(2., 3., 4.);
        let inverse = transform.inverse().unwrap();
        let vector = Tuple::vector(-4., 6., 8.);

        let expected = Tuple::vector(-2., 2., 2.);

        let actual = inverse * vector;

        assert_eq!(expected, actual);
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = scale(-1., 1., 1.);
        let point = Tuple::point(2., 3., 4.);

        let expected = Tuple::point(-2., 3., 4.);

        let actual = transform * point;

        assert_eq!(expected, actual);
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let point = Tuple::point(0., 1., 0.);
        let half_quarter = rotate(PI / 4., Axis::X);
        let full_quarter = rotate(PI / 2., Axis::X);

        let expected_half = Tuple::point(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.);
        let expected_full = Tuple::point(0., 0., 1.);

        let actual_half = half_quarter * point;
        let actual_full = full_quarter * point;

        assert_eq!(expected_half, actual_half);
        assert_eq!(expected_full, actual_full);
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opposite_direction() {
        let point = Tuple::point(0., 1., 0.);
        let half_quarter = rotate(PI / 4., Axis::X);
        let inverse = half_quarter.inverse().unwrap();

        let expected = Tuple::point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);

        let actual = inverse * point;

        assert_eq!(expected, actual);
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let point = Tuple::point(0., 0., 1.);
        let half_quarter = rotate(PI / 4., Axis::Y);
        let full_quarter = rotate(PI / 2., Axis::Y);

        let expected_half = Tuple::point(2_f64.sqrt() / 2., 0., 2_f64.sqrt() / 2.);
        let expected_full = Tuple::point(1., 0., 0.);

        let actual_half = half_quarter * point;
        let actual_full = full_quarter * point;

        assert_eq!(expected_half, actual_half);
        assert_eq!(expected_full, actual_full);
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let point = Tuple::point(0., 1., 0.);
        let half_quarter = rotate(PI / 4., Axis::Z);
        let full_quarter = rotate(PI / 2., Axis::Z);

        let expected_half = Tuple::point(-2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 0.);
        let expected_full = Tuple::point(-1., 0., 0.);

        let actual_half = half_quarter * point;
        let actual_full = full_quarter * point;

        assert_eq!(expected_half, actual_half);
        assert_eq!(expected_full, actual_full);
    }
}