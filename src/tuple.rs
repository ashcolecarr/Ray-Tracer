use super::near_eq;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        near_eq(self.x, other.x) && near_eq(self.y, other.y) && 
            near_eq(self.z, other.z) && near_eq(self.w, other.w)
    }
}

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1. }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0. }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        Self {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: self.w / self.magnitude()
        }
    }

    pub fn dot(&self, other: Self) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::vector((self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z), (self.x * other.y) - (self.y * other.x))
    }

    pub fn is_point(&self) -> bool {
        near_eq(1.0, self.w)
    }
    
    pub fn is_vector(&self) -> bool {
        near_eq(0.0, self.w)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::near_eq;

    #[test]
    fn tuple_with_w1_is_point() {
        let expected_x = 4.3;
        let expected_y = -4.2;
        let expected_z = 3.1;
        let expected_w = 1.0;
        
        let actual = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0 };

        assert!(near_eq(expected_x, actual.x));
        assert!(near_eq(expected_y, actual.y));
        assert!(near_eq(expected_z, actual.z));
        assert!(near_eq(expected_w, actual.w));
        assert!(actual.is_point());
        assert!(!actual.is_vector());
    }

    #[test]
    fn tuple_with_w0_is_vector() {
        let expected_x = 4.3;
        let expected_y = -4.2;
        let expected_z = 3.1;
        let expected_w = 0.0;
        
        let actual = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.0 };

        assert!(near_eq(expected_x, actual.x));
        assert!(near_eq(expected_y, actual.y));
        assert!(near_eq(expected_z, actual.z));
        assert!(near_eq(expected_w, actual.w));
        assert!(!actual.is_point());
        assert!(actual.is_vector());
    }

    #[test]
    fn point_function_creates_tuples_with_w1() {
        let expected = Tuple { x: 4., y: -4., z: 3., w: 1. };

        let actual = Tuple::point(4., -4., 3.);

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn point_function_creates_tuples_with_w0() {
        let expected = Tuple { x: 4., y: -4., z: 3., w: 0. };

        let actual = Tuple::vector(4., -4., 3.);

        assert_eq!(expected, actual);
    }

    #[test]
    fn adding_two_tuples() {
        let tuple1 = Tuple { x: 3., y: -2., z: 5., w: 1. };
        let tuple2 = Tuple { x: -2., y: 3., z: 1., w: 0. };
        
        let expected = Tuple { x: 1., y: 1., z: 6., w: 1. };

        let actual = tuple1 + tuple2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_two_points() {
        let point1 = Tuple::point(3., 2., 1.);
        let point2 = Tuple::point(5., 6., 7.);
        
        let expected = Tuple::vector(-2., -4., -6.);

        let actual = point1 - point2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_vector_from_point() {
        let point = Tuple::point(3., 2., 1.);
        let vector = Tuple::vector(5., 6., 7.);
        
        let expected = Tuple::point(-2., -4., -6.);

        let actual = point - vector;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_two_vectors() {
        let vector1 = Tuple::vector(3., 2., 1.);
        let vector2 = Tuple::vector(5., 6., 7.);
        
        let expected = Tuple::vector(-2., -4., -6.);

        let actual = vector1 - vector2;

        assert_eq!(expected, actual);
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = Tuple::vector(0., 0., 0.);
        let vector = Tuple::vector(1., -2., 3.);

        let expected = Tuple::vector(-1., 2., -3.);

        let actual = zero - vector;

        assert_eq!(expected, actual);
    }

    #[test]
    fn negating_tuple() {
        let tuple = Tuple { x: 1., y: -2., z: 3., w: -4. };

        let expected = Tuple { x: -1., y: 2., z: -3., w: 4. };

        let actual = -tuple;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let tuple = Tuple { x: 1., y: -2., z: 3., w: -4. };

        let expected = Tuple { x: 3.5, y: -7., z: 10.5, w: -14. };

        let actual = tuple * 3.5;

        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_tuple_by_fraction() {
        let tuple = Tuple { x: 1., y: -2., z: 3., w: -4. };

        let expected = Tuple { x: 0.5, y: -1., z: 1.5, w: -2. };

        let actual = tuple * 0.5;

        assert_eq!(expected, actual);
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let tuple = Tuple { x: 1., y: -2., z: 3., w: -4. };

        let expected = Tuple { x: 0.5, y: -1., z: 1.5, w: -2. };

        let actual = tuple / 2.;

        assert_eq!(expected, actual);
    }

    #[test]
    fn computing_magnitude_of_vector1() {
        let vector = Tuple::vector(1., 0., 0.);

        let expected = 1.;

        let actual = vector.magnitude();

        assert_eq!(expected, actual);
    }

    #[test]
    fn computing_magnitude_of_vector2() {
        let vector = Tuple::vector(0., 1., 0.);

        let expected = 1.;

        let actual = vector.magnitude();

        assert_eq!(expected, actual);
    }

    #[test]
    fn computing_magnitude_of_vector3() {
        let vector = Tuple::vector(0., 0., 1.);

        let expected = 1.;

        let actual = vector.magnitude();

        assert_eq!(expected, actual);
    }

    #[test]
    fn computing_magnitude_of_vector4() {
        let vector = Tuple::vector(1., 2., 3.);

        let expected = 14_f64.sqrt();

        let actual = vector.magnitude();

        assert_eq!(expected, actual);
    }

    #[test]
    fn computing_magnitude_of_vector() {
        let vector = Tuple::vector(-1., -2., -3.);

        let expected = 14_f64.sqrt();

        let actual = vector.magnitude();

        assert_eq!(expected, actual);
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let vector = Tuple::vector(4., 0., 0.);

        let expected = Tuple::vector(1., 0., 0.);

        let actual = vector.normalize();

        assert_eq!(expected, actual);
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let vector = Tuple::vector(1., 2., 3.);

        let expected = Tuple::vector(0.26726, 0.53452, 0.80178);

        let actual = vector.normalize();

        assert_eq!(expected, actual);
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let vector = Tuple::vector(1., 2., 3.);
        let normal = vector.normalize();

        let expected = 1.;

        let actual = normal.magnitude();

        assert_eq!(expected, actual);
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let tuple1 = Tuple::vector(1., 2., 3.);
        let tuple2 = Tuple::vector(2., 3., 4.);

        let expected = 20.;

        let actual = tuple1.dot(tuple2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let vector1 = Tuple::vector(1., 2., 3.);
        let vector2 = Tuple::vector(2., 3., 4.);

        let expected1 = Tuple::vector(-1., 2., -1.);
        let expected2 = Tuple::vector(1., -2., 1.);

        let actual1 = vector1.cross(vector2);
        let actual2 = vector2.cross(vector1);

        assert_eq!(expected1, actual1);
        assert_eq!(expected2, actual2);
    }
}