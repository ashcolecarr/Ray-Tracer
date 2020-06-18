use super::EPSILON;
use super::matrix::Matrix;
use super::near_eq;
use super::ray::Ray;
use super::tuple::Tuple;
use std::f64::INFINITY;

#[derive(Debug, Clone)]
pub struct Bound {
    pub minimum: Tuple,
    pub maximum: Tuple,    
}

impl PartialEq for Bound {
    fn eq(&self, other: &Self) -> bool {
        self.minimum == other.minimum && self.maximum == other.maximum
    }
}

impl Bound {
    pub fn bounding_box_empty() -> Self {
        Self {
            minimum: Tuple::point(INFINITY, INFINITY, INFINITY),
            maximum: Tuple::point(-INFINITY, -INFINITY, -INFINITY),
        }
    }

    pub fn bounding_box_init(minimum: Tuple, maximum: Tuple) -> Self {
        Self {
            minimum,
            maximum,
        }
    }

    pub fn add_point(&mut self, point: Tuple) {
        if point.x < self.minimum.x { self.minimum.x = point.x };
        if point.y < self.minimum.y { self.minimum.y = point.y };
        if point.z < self.minimum.z { self.minimum.z = point.z };
        if point.x > self.maximum.x { self.maximum.x = point.x };
        if point.y > self.maximum.y { self.maximum.y = point.y };
        if point.z > self.maximum.z { self.maximum.z = point.z };
    }

    pub fn add_box(&mut self, other: Self) {
        self.add_point(other.minimum);
        self.add_point(other.maximum);
    }

    pub fn box_contains_point(&self, point: Tuple) -> bool {
        let contains_x = near_eq(self.minimum.x, point.x) || near_eq(self.maximum.x, point.x) || 
            (self.minimum.x < point.x && point.x < self.maximum.x); 
        let contains_y = near_eq(self.minimum.y, point.y) || near_eq(self.maximum.y, point.y) || 
            (self.minimum.y < point.y && point.y < self.maximum.y); 
        let contains_z = near_eq(self.minimum.z, point.z) || near_eq(self.maximum.z, point.z) || 
            (self.minimum.z < point.z && point.z < self.maximum.z); 

        contains_x && contains_y && contains_z
    }

    pub fn box_contains_box(&self, other: Self) -> bool {
        self.box_contains_point(other.minimum) && self.box_contains_point(other.maximum)
    }

    pub fn transform(&self, matrix: Matrix) -> Self {
        let points = vec![
            self.minimum,
            Tuple::point(self.minimum.x, self.minimum.y, self.maximum.z),
            Tuple::point(self.minimum.x, self.maximum.y, self.minimum.z),
            Tuple::point(self.minimum.x, self.maximum.y, self.maximum.z),
            Tuple::point(self.maximum.x, self.minimum.y, self.minimum.z),
            Tuple::point(self.maximum.x, self.minimum.y, self.maximum.z),
            Tuple::point(self.maximum.x, self.maximum.y, self.minimum.z),
            self.maximum
        ];

        let mut new_box = Self::bounding_box_empty();

        for point in points {
            new_box.add_point(matrix.clone() * point);
        }

        new_box
    }

    pub fn intersects(&self, ray: Ray) -> bool {
        let (xtmin, xtmax) = Bound::check_axis(ray.origin.x, ray.direction.x, self.minimum.x, self.maximum.x);
        let (ytmin, ytmax) = Bound::check_axis(ray.origin.y, ray.direction.y, self.minimum.y, self.maximum.y);
        let (ztmin, ztmax) = Bound::check_axis(ray.origin.z, ray.direction.z, self.minimum.z, self.maximum.z);

        let tmin = vec![xtmin, ytmin, ztmin].iter().fold(0./0., |max, &n| f64::max(max, n));
        let tmax = vec![xtmax, ytmax, ztmax].iter().fold(0./0., |min, &n| f64::min(min, n));

        if tmin > tmax {
            return false;
        }

        true
    }

    fn check_axis(origin: f64, direction: f64, minimum: f64, maximum: f64) -> (f64, f64) {
        let tmin_numerator = minimum - origin;
        let tmax_numerator = maximum - origin;

        let (tmin, tmax) = if direction.abs() >= EPSILON {
            (tmin_numerator / direction, tmax_numerator / direction)
        } else {
            (tmin_numerator * INFINITY, tmax_numerator * INFINITY)
        };

        if tmin > tmax {
            // Swap the values.
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }

    pub fn split_bounds(&self) -> (Self, Self) {
        let dx = (self.maximum.x - self.minimum.x).abs();
        let dy = (self.maximum.y - self.minimum.y).abs();
        let dz = (self.maximum.z - self.minimum.z).abs();
        
        let greatest = vec![dx, dy, dz].iter().fold(0./0., |max, &n| f64::max(max, n));

        let (mut x0, mut y0, mut z0) = (self.minimum.x, self.minimum.y, self.minimum.z);
        let (mut x1, mut y1, mut z1) = (self.maximum.x, self.maximum.y, self.maximum.z);

        if near_eq(greatest, dx) {
            x0 = x0 + dx / 2.;
            x1 = x0;
        } else if near_eq(greatest, dy) {
            y0 = y0 + dy / 2.;
            y1 = y0;
        } else {
            z0 = z0 + dz / 2.;
            z1 = z0;
        }

        let mid_min = Tuple::point(x0, y0, z0);
        let mid_max = Tuple::point(x1, y1, z1);
        
        let left = Bound::bounding_box_init(self.minimum, mid_max);
        let right = Bound::bounding_box_init(mid_min, self.maximum);

        (left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::ray::Ray;
    use super::super::transformation::*;
    use std::f64::consts::PI;
    use std::f64::INFINITY;

    #[test]
    fn creating_empty_bounding_box() {
        let expected_minimum = Tuple::point(INFINITY, INFINITY, INFINITY);
        let expected_maximum = Tuple::point(-INFINITY, -INFINITY, -INFINITY);

        let actual = Bound::bounding_box_empty();

        assert_eq!(expected_minimum.x, actual.minimum.x);
        assert_eq!(expected_minimum.y, actual.minimum.y);
        assert_eq!(expected_minimum.z, actual.minimum.z);
        assert_eq!(expected_maximum.x, actual.maximum.x);
        assert_eq!(expected_maximum.y, actual.maximum.y);
        assert_eq!(expected_maximum.z, actual.maximum.z);
    }

    #[test]
    fn creating_bounding_box_with_volume() {
        let minimum = Tuple::point(-1., -2., -3.);
        let maximum = Tuple::point(3., 2., 1.);

        let expected_minimum = minimum;
        let expected_maximum = maximum;

        let actual = Bound::bounding_box_init(minimum, maximum);

        assert_eq!(expected_minimum, actual.minimum);
        assert_eq!(expected_maximum, actual.maximum);
    }

    #[test]
    fn adding_points_to_empty_bounding_box() {
        let point1 = Tuple::point(-5., 2., 0.);
        let point2 = Tuple::point(7., 0., -3.);
        let mut box1 = Bound::bounding_box_empty();
        box1.add_point(point1);
        box1.add_point(point2);

        let expected_minimum = Tuple::point(-5., 0., -3.);
        let expected_maximum = Tuple::point(7., 2., 0.);

        let actual_minimum = box1.minimum;
        let actual_maximum = box1.maximum;

        assert_eq!(expected_minimum, actual_minimum);
        assert_eq!(expected_maximum, actual_maximum);
    }

    #[test]
    fn adding_one_bounding_box_to_another() {
        let mut box1 = Bound::bounding_box_init(Tuple::point(-5., -2., 0.), 
            Tuple::point(7., 4., 4.)); 
        let box2 = Bound::bounding_box_init(Tuple::point(8., -7., -2.), 
            Tuple::point(14., 2., 8.)); 
        box1.add_box(box2);

        let expected_minimum = Tuple::point(-5., -7., -2.);
        let expected_maximum = Tuple::point(14., 4., 8.);

        let actual_minimum = box1.minimum;
        let actual_maximum = box1.maximum;

        assert_eq!(expected_minimum, actual_minimum);
        assert_eq!(expected_maximum, actual_maximum);
    }

    #[test]
    fn checking_to_see_if_box_contains_given_point() {
        let box1 = Bound::bounding_box_init(Tuple::point(5., -2., 0.), 
            Tuple::point(11., 4., 7.));
        let points = vec![Tuple::point(5., -2., 0.), Tuple::point(11., 4., 7.),
            Tuple::point(8., 1., 3.), Tuple::point(3., 0., 3.),
            Tuple::point(8., -4., 3.), Tuple::point(8., 1., -1.),
            Tuple::point(13., 1., 3.), Tuple::point(8., 5., 3.),
            Tuple::point(8., 1., 8.)];

        let expecteds = vec![true, true, true, false, false, false,
            false, false, false];
        
        for source in expecteds.iter().zip(points) {
            let (expected, point) = source;
            let actual = box1.box_contains_point(point);

            assert_eq!(*expected, actual);
        }
    }

    #[test]
    fn checking_to_see_if_box_contains_given_box() {
        let box1 = Bound::bounding_box_init(Tuple::point(5., -2., 0.), Tuple::point(11., 4., 7.));
        let boxes = vec![
            Bound::bounding_box_init(Tuple::point(5., -2., 0.), Tuple::point(11., 4., 7.)),
            Bound::bounding_box_init(Tuple::point(6., -1., 1.), Tuple::point(10., 3., 6.)),
            Bound::bounding_box_init(Tuple::point(4., -3., -1.), Tuple::point(10., 3., 6.)),
            Bound::bounding_box_init(Tuple::point(6., -1., 1.), Tuple::point(12., 5., 8.)),
        ];

        let expecteds = vec![true, true, false, false];

        for source in expecteds.iter().zip(boxes) {
            let (expected, box2) = source;
            let actual = box1.box_contains_box(box2);

            assert_eq!(*expected, actual);
        }
    }

    #[test]
    fn transforming_bounding_box() {
        let box1 = Bound::bounding_box_init(Tuple::point(-1., -1., -1.), 
            Tuple::point(1., 1., 1.));
        let matrix = rotate(PI / 4., Axis::X) * rotate(PI / 4., Axis::Y);

        let expected_minimum = Tuple::point(-1.41421, -1.7071, -1.7071);
        let expected_maximum = Tuple::point(1.41421, 1.7071, 1.7071);

        let actual = box1.transform(matrix);

        assert_eq!(expected_minimum, actual.minimum);
        assert_eq!(expected_maximum, actual.maximum);
    }

    #[test]
    fn intersecting_ray_with_bounding_box_at_origin() {
        let box1 = Bound::bounding_box_init(Tuple::point(-1., -1., -1.), Tuple::point(1., 1., 1.));
        let origins = vec![Tuple::point(5., 0.5, 0.), Tuple::point(-5., 0.5, 0.), 
            Tuple::point(0.5, 5., 0.), Tuple::point(0.5, -5., 0.), 
            Tuple::point(0.5, 0., 5.), Tuple::point(0.5, 0., -5.), 
            Tuple::point(0., 0.5, 0.), Tuple::point(-2., 0., 0.), 
            Tuple::point(0., -2., 0.), Tuple::point(0., 0., -2.), 
            Tuple::point(2., 0., 2.), Tuple::point(0., 2., 2.), 
            Tuple::point(2., 2., 0.)];
        let directions = vec![Tuple::vector(-1., 0., 0.), Tuple::vector(1., 0., 0.), 
            Tuple::vector(0., -1., 0.), Tuple::vector(0., 1., 0.), 
            Tuple::vector(0., 0., -1.), Tuple::vector(0., 0., 1.), 
            Tuple::vector(0., 0., 1.), Tuple::vector(2., 4., 6.), 
            Tuple::vector(6., 2., 4.), Tuple::vector(4., 6., 2.), 
            Tuple::vector(0., 0., -1.), Tuple::vector(0., -1., 0.), 
            Tuple::vector(-1., 0., 0.)];
        
        let expecteds = vec![true, true, true, true, true, true, true, 
            false, false, false, false, false, false];

        for source in expecteds.iter().zip(origins).zip(directions) {
            let ((expected, origin), direction) = source;

            let direction = direction.normalize();
            let ray = Ray::new(origin, direction);

            let actual = box1.intersects(ray);

            assert_eq!(*expected, actual);
        }
    }

    #[test]
    fn intersecting_ray_with_noncubic_bounding_box() {
        let box1 = Bound::bounding_box_init(Tuple::point(5., -2., 0.), Tuple::point(11., 4., 7.));
        let origins = vec![Tuple::point(1.5, 1., 2.), Tuple::point(-5., -1., 4.), 
            Tuple::point(7., 6., 5.), Tuple::point(9., -5., 6.), 
            Tuple::point(8., 2., 1.2), Tuple::point(6., 0., -5.), 
            Tuple::point(8., 1., 3.5), Tuple::point(9., -1., -8.), 
            Tuple::point(8., 3., -4.), Tuple::point(9., -1., -2.), 
            Tuple::point(4., 0., 9.),  Tuple::point(8., 6., -1.), 
            Tuple::point(1.2, 5., 4.)];
        let directions = vec![Tuple::vector(-1., 0., 0.), Tuple::vector(1., 0., 0.), 
            Tuple::vector(0., -1., 0.), Tuple::vector(0., 1., 0.), 
            Tuple::vector(0., 0., -1.), Tuple::vector(0., 0., 1.), 
            Tuple::vector(0., 0., 1.), Tuple::vector(2., 4., 6.), 
            Tuple::vector(6., 2., 4.), Tuple::vector(4., 6., 2.), 
            Tuple::vector(0., 0., -1.), Tuple::vector(0., -1., 0.),
            Tuple::vector(-1., 0., 0.)];
        
        let expecteds = vec![true, true, true, true, true, true, true, 
            false, false, false, false, false, false];

        for source in expecteds.iter().zip(origins).zip(directions) {
            let ((expected, origin), direction) = source;

            let direction = direction.normalize();
            let ray = Ray::new(origin, direction);

            let actual = box1.intersects(ray);

            assert_eq!(*expected, actual);
        }
    }

    #[test]
    fn splitting_perfect_cube() {
        let box1 = Bound::bounding_box_init(Tuple::point(-1., -4., -5.), Tuple::point(9., 6., 5.));

        let expected_left = Bound::bounding_box_init(Tuple::point(-1., -4., -5.), Tuple::point(4., 6., 5.));
        let expected_right = Bound::bounding_box_init(Tuple::point(4., -4., -5.), Tuple::point(9., 6., 5.));

        let (actual_left, actual_right) = box1.split_bounds();

        assert_eq!(expected_left.minimum, actual_left.minimum);
        assert_eq!(expected_left.maximum, actual_left.maximum);
        assert_eq!(expected_right.minimum, actual_right.minimum);
        assert_eq!(expected_right.maximum, actual_right.maximum);
    }

    #[test]
    fn splitting_xwide_box() {
        let box1 = Bound::bounding_box_init(Tuple::point(-1., -2., -3.), Tuple::point(9., 5.5, 3.));

        let expected_left = Bound::bounding_box_init(Tuple::point(-1., -2., -3.), Tuple::point(4., 5.5, 3.));
        let expected_right = Bound::bounding_box_init(Tuple::point(4., -2., -3.), Tuple::point(9., 5.5, 3.));

        let (actual_left, actual_right) = box1.split_bounds();

        assert_eq!(expected_left.minimum, actual_left.minimum);
        assert_eq!(expected_left.maximum, actual_left.maximum);
        assert_eq!(expected_right.minimum, actual_right.minimum);
        assert_eq!(expected_right.maximum, actual_right.maximum);
    }

    #[test]
    fn splitting_ywide_box() {
        let box1 = Bound::bounding_box_init(Tuple::point(-1., -2., -3.), Tuple::point(5., 8., 3.));

        let expected_left = Bound::bounding_box_init(Tuple::point(-1., -2., -3.), Tuple::point(5., 3., 3.));
        let expected_right = Bound::bounding_box_init(Tuple::point(-1., 3., -3.), Tuple::point(5., 8., 3.));

        let (actual_left, actual_right) = box1.split_bounds();

        assert_eq!(expected_left.minimum, actual_left.minimum);
        assert_eq!(expected_left.maximum, actual_left.maximum);
        assert_eq!(expected_right.minimum, actual_right.minimum);
        assert_eq!(expected_right.maximum, actual_right.maximum);
    }
    #[test]
    fn splitting_zwide_box() {
        let box1 = Bound::bounding_box_init(Tuple::point(-1., -2., -3.), Tuple::point(5., 3., 7.));

        let expected_left = Bound::bounding_box_init(Tuple::point(-1., -2., -3.), Tuple::point(5., 3., 2.));
        let expected_right = Bound::bounding_box_init(Tuple::point(-1., -2., 2.), Tuple::point(5., 3., 7.));

        let (actual_left, actual_right) = box1.split_bounds();

        assert_eq!(expected_left.minimum, actual_left.minimum);
        assert_eq!(expected_left.maximum, actual_left.maximum);
        assert_eq!(expected_right.minimum, actual_right.minimum);
        assert_eq!(expected_right.maximum, actual_right.maximum);
    }
}