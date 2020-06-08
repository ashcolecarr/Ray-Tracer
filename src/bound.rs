use super::near_eq;
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
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let mut bounding_box = Bound::bounding_box_empty();
        bounding_box.add_point(point1);
        bounding_box.add_point(point2);

        let expected_minimum = Tuple::point(-5., 0., -3.);
        let expected_maximum = Tuple::point(7., 2., 0.);

        let actual_minimum = bounding_box.minimum;
        let actual_maximum = bounding_box.maximum;

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
        let bounding_box = Bound::bounding_box_init(Tuple::point(5., -2., 0.), 
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
            let actual = bounding_box.box_contains_point(point);

            assert_eq!(*expected, actual);
        }
    }

    #[test]
    fn checking_to_see_if_box_contains_given_box() {
        let box1 = Bound::bounding_box_init(Tuple::point(5., -2., 0.), Tuple::point(11., 4., 7.));
        let boxes = vec![
            Bound::bounding_box_init(Tuple::point(5., -2., 0.), Tuple::point(11., 4., 7.)),
            Bound::bounding_box_init(Tuple::point(5., -2., 0.), Tuple::point(11., 4., 7.)),
            Bound::bounding_box_init(Tuple::point(5., -2., 0.), Tuple::point(11., 4., 7.)),
            Bound::bounding_box_init(Tuple::point(5., -2., 0.), Tuple::point(11., 4., 7.)),
        ];

        let expecteds = vec![true, true, false, false];

        for source in expecteds.iter().zip(boxes) {
            let (expected, box2) = source;
            let actual = box1.box_contains_box(box2);

            assert_eq!(*expected, actual);
        }
    }
}