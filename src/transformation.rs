use super::matrix::Matrix;
use super::tuple::Tuple;

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

pub fn shear(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
    let mut matrix = Matrix::identity(4);
    matrix[0][1] = x_y;
    matrix[0][2] = x_z;
    matrix[1][0] = y_x;
    matrix[1][2] = y_z;
    matrix[2][0] = z_x;
    matrix[2][1] = z_y;

    matrix
}

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix {
    let forward = (to - from).normalize();
    let up_normal = up.normalize();
    let left = forward.cross(up_normal);
    let true_up = left.cross(forward);

    let orientation = Matrix::new(4, 4, vec![
        left.x, left.y, left.z, 0., 
        true_up.x, true_up.y, true_up.z, 0.,
        -forward.x, -forward.y, -forward.z, 0.,
        0., 0., 0., 1.]);
    
    orientation * translate(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::ORIGIN;
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

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = shear(1., 0., 0., 0., 0., 0.);
        let point = Tuple::point(2., 3., 4.);

        let expected = Tuple::point(5., 3., 4.);

        let actual = transform * point;

        assert_eq!(expected, actual);
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = shear(0., 1., 0., 0., 0., 0.);
        let point = Tuple::point(2., 3., 4.);

        let expected = Tuple::point(6., 3., 4.);

        let actual = transform * point;

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = shear(0., 0., 1., 0., 0., 0.);
        let point = Tuple::point(2., 3., 4.);

        let expected = Tuple::point(2., 5., 4.);

        let actual = transform * point;

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = shear(0., 0., 0., 1., 0., 0.);
        let point = Tuple::point(2., 3., 4.);

        let expected = Tuple::point(2., 7., 4.);

        let actual = transform * point;

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = shear(0., 0., 0., 0., 1., 0.);
        let point = Tuple::point(2., 3., 4.);

        let expected = Tuple::point(2., 3., 6.);

        let actual = transform * point;

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = shear(0., 0., 0., 0., 0., 1.);
        let point = Tuple::point(2., 3., 4.);

        let expected = Tuple::point(2., 3., 7.);

        let actual = transform * point;

        assert_eq!(expected, actual);
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let point = Tuple::point(1., 0., 1.);
        let rotation_x = rotate(PI / 2., Axis::X);
        let scaling = scale(5., 5., 5.);
        let translation = translate(10., 5., 7.);

        let expected_rotation = Tuple::point(1., -1., 0.);
        let actual_rotation = rotation_x * point;
        assert_eq!(expected_rotation, actual_rotation);

        let expected_scaling = Tuple::point(5., -5., 0.);
        let actual_scaling = scaling * actual_rotation;
        assert_eq!(expected_scaling, actual_scaling);

        let expected_translation = Tuple::point(15., 0., 7.);
        let actual_translation = translation * actual_scaling;
        assert_eq!(expected_translation, actual_translation);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let point = Tuple::point(1., 0., 1.);
        let rotation_x = rotate(PI / 2., Axis::X);
        let scaling = scale(5., 5., 5.);
        let translation = translate(10., 5., 7.);
        let transformation = translation * scaling * rotation_x;

        let expected = Tuple::point(15., 0., 7.);

        let actual = transformation * point;

        assert_eq!(expected, actual);
    }

    #[test]
    fn transformation_matrix_for_default_orientation() {
        let from = ORIGIN;
        let to = Tuple::point(0., 0., -1.);
        let up = Tuple::vector(0., 1., 0.);

        let expected = Matrix::identity(4);

        let actual = view_transform(from, to, up);

        assert_eq!(expected, actual);
    }

    #[test]
    fn view_transformation_matrix_looking_in_positive_z_direction() {
        let from = ORIGIN;
        let to = Tuple::point(0., 0., 1.);
        let up = Tuple::vector(0., 1., 0.);

        let expected = scale(-1., 1., -1.);

        let actual = view_transform(from, to, up);

        assert_eq!(expected, actual);
    }

    #[test]
    fn view_transformation_moves_world() {
        let from = Tuple::point(0., 0., 8.);
        let to = ORIGIN;
        let up = Tuple::vector(0., 1., 0.);

        let expected = translate(0., 0., -8.);

        let actual = view_transform(from, to, up);

        assert_eq!(expected, actual);
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Tuple::point(1., 3., 2.);
        let to = Tuple::point(4., -2., 8.);
        let up = Tuple::vector(1., 1., 0.);

        let values = vec![-0.50709, 0.50709, 0.67612, -2.36643, 
            0.76772, 0.60609, 0.12122, -2.82843,
            -0.35857, 0.59761, -0.71714, 0.,
            0., 0., 0., 1.]; 
        let expected = Matrix::new(4, 4, values);

        let actual = view_transform(from, to, up);

        assert_eq!(expected, actual);
    }
}