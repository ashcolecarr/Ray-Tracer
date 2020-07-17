use super::matrix::Matrix;
use super::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.direction == other.direction
    }
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(&self, transformation: &Matrix) -> Self {
        let origin_transform = transformation.clone() * self.origin;
        let direction_transform = transformation.clone() * self.direction;

        Ray::new(origin_transform, direction_transform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::transformation::*;
    use super::super::tuple::Tuple;

    #[test]
    fn creating_and_querying_ray() {
        let origin = Tuple::point(1., 2., 3.);
        let direction = Tuple::vector(4., 5., 6.);

        let expected_origin = origin;
        let expected_direction = direction;

        let actual = Ray::new(origin, direction);

        assert_eq!(expected_origin, actual.origin);
        assert_eq!(expected_direction, actual.direction);
    }

    #[test]
    fn computing_point_from_distance() {
        let ray = Ray::new(Tuple::point(2., 3., 4.), Tuple::vector(1., 0., 0.));

        let expected_position1 = Tuple::point(2., 3., 4.);
        let expected_position2 = Tuple::point(3., 3., 4.);
        let expected_position3 = Tuple::point(1., 3., 4.);
        let expected_position4 = Tuple::point(4.5, 3., 4.);
        
        let actual_position1 = ray.position(0.);
        let actual_position2 = ray.position(1.);
        let actual_position3 = ray.position(-1.);
        let actual_position4 = ray.position(2.5);

        assert_eq!(expected_position1, actual_position1);
        assert_eq!(expected_position2, actual_position2);
        assert_eq!(expected_position3, actual_position3);
        assert_eq!(expected_position4, actual_position4);
    }

    #[test]
    fn translating_ray() {
        let ray = Ray::new(Tuple::point(1., 2., 3.), Tuple::vector(0., 1., 0.));
        let transform = translate(3., 4., 5.);

        let expected = Ray::new(Tuple::point(4., 6., 8.), Tuple::vector(0., 1., 0.));

        let actual = ray.transform(&transform);

        assert_eq!(expected.origin, actual.origin);
        assert_eq!(expected.direction, actual.direction);
    }

    #[test]
    fn scaling_ray() {
        let ray = Ray::new(Tuple::point(1., 2., 3.), Tuple::vector(0., 1., 0.));
        let transform = scale(2., 3., 4.);

        let expected = Ray::new(Tuple::point(2., 6., 12.), Tuple::vector(0., 3., 0.));

        let actual = ray.transform(&transform);

        assert_eq!(expected.origin, actual.origin);
        assert_eq!(expected.direction, actual.direction);
    }
}