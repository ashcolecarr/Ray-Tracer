use super::EPSILON;
use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
use super::ray::Ray;
use super::shape::Shape;
use super::tuple::Tuple;
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Debug, Clone)]
pub struct Plane {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform &&
            self.material == other.material
    }
}

impl Plane {
    pub fn new() -> Self {
        static ID_COUNT: AtomicI32 = AtomicI32::new(1);

        Self {
            id: ID_COUNT.fetch_add(1, Ordering::Relaxed),
            transform: Matrix::identity(4),
            material: Default::default(),
        }
    }
    
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        if ray.direction.y.abs() < EPSILON {
            vec![]
        } else {
            let t = -ray.origin.y / ray.direction.y;
            vec![Intersection::new(t, Shape::Plane(self.clone()))]
        }
    }

    pub fn normal_at(&self, _world_point: Tuple) -> Tuple {
        // Every point on a plane has the same normal.
        Tuple::vector(0., 1., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::ORIGIN;
    use super::super::ray::Ray;
    use super::super::shape::{Shape, Actionable};
    use super::super::tuple::Tuple;

    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let plane = Shape::Plane(Plane::new());

        let expected1 = Tuple::vector(0., 1., 0.);
        let expected2 = Tuple::vector(0., 1., 0.);
        let expected3 = Tuple::vector(0., 1., 0.);

        let actual1 = plane.normal_at(ORIGIN);
        let actual2 = plane.normal_at(Tuple::point(10., 0., -10.));
        let actual3 = plane.normal_at(Tuple::point(-5., 0., 150.));

        assert_eq!(expected1, actual1);
        assert_eq!(expected2, actual2);
        assert_eq!(expected3, actual3);
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let plane = Shape::Plane(Plane::new());
        let ray = Ray::new(Tuple::point(0., 10., 0.), Tuple::vector(0., 0., 1.));

        let actual = plane.intersect(ray);

        assert!(actual.is_empty());
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let plane = Shape::Plane(Plane::new());
        let ray = Ray::new(ORIGIN, Tuple::vector(0., 0., 1.));

        let actual = plane.intersect(ray);

        assert!(actual.is_empty());
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let plane = Shape::Plane(Plane::new());
        let ray = Ray::new(Tuple::point(0., 1., 0.), Tuple::vector(0., -1., 0.));

        let expected_count = 1;
        let expected_t = 1.;
        let expected_object = plane.clone();

        let actual = plane.intersect(ray);

        assert_eq!(expected_count, actual.len());
        assert_eq!(expected_t, actual[0].t);
        assert_eq!(expected_object, actual[0].object);
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let plane = Shape::Plane(Plane::new());
        let ray = Ray::new(Tuple::point(0., -1., 0.), Tuple::vector(0., 1., 0.));

        let expected_count = 1;
        let expected_t = 1.;
        let expected_object = plane.clone();

        let actual = plane.intersect(ray);

        assert_eq!(expected_count, actual.len());
        assert_eq!(expected_t, actual[0].t);
        assert_eq!(expected_object, actual[0].object);
    }
}