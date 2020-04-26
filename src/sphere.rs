use super::intersection::Intersection;
use super::ray::Ray;
use super::tuple::Tuple;
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Debug, Clone)]
pub struct Sphere {
    id: i32,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Sphere {
    pub fn new() -> Self {
        static ID_COUNT:AtomicI32 = AtomicI32::new(1);

        Self {
            id: ID_COUNT.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Tuple::point(0., 0., 0.);

        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        vec![
            Intersection::new(t1, self.clone()),
            Intersection::new(t2, self.clone()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::near_eq;
    use super::super::ray::Ray;
    use super::super::tuple::Tuple;

    #[test]
    fn sphere_ids_are_unique() {
        let actual1 = Sphere::new();
        let actual2 = Sphere::new();
        let actual3 = Sphere::new();

        assert_ne!(actual1.get_id(), actual2.get_id());
        assert_ne!(actual1.get_id(), actual3.get_id());
        assert_ne!(actual2.get_id(), actual3.get_id());
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let sphere = Sphere::new();
        
        let expected_count = 2;
        let expected_0 = 4.;
        let expected_1 = 6.;

        let actual = sphere.intersect(ray);

        assert_eq!(expected_count, actual.len());
        assert!(near_eq(expected_0, actual[0].t));
        assert!(near_eq(expected_1, actual[1].t));
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let ray = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let sphere = Sphere::new();
        
        let expected_count = 2;
        let expected_0 = 5.;
        let expected_1 = 5.;

        let actual = sphere.intersect(ray);

        assert_eq!(expected_count, actual.len());
        assert!(near_eq(expected_0, actual[0].t));
        assert!(near_eq(expected_1, actual[1].t));
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let sphere = Sphere::new();
        
        let actual = sphere.intersect(ray);

        assert!(actual.is_empty());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let sphere = Sphere::new();
        
        let expected_count = 2;
        let expected_0 = -1.;
        let expected_1 = 1.;

        let actual = sphere.intersect(ray);

        assert_eq!(expected_count, actual.len());
        assert!(near_eq(expected_0, actual[0].t));
        assert!(near_eq(expected_1, actual[1].t));
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let sphere = Sphere::new();
        
        let expected_count = 2;
        let expected_0 = -6.;
        let expected_1 = -4.;

        let actual = sphere.intersect(ray);

        assert_eq!(expected_count, actual.len());
        assert!(near_eq(expected_0, actual[0].t));
        assert!(near_eq(expected_1, actual[1].t));
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let sphere = Sphere::new();

        let expected_count = 2;
        let expected_object1 = sphere.clone();
        let expected_object2 = sphere.clone();

        let actual = sphere.intersect(ray);

        assert_eq!(expected_count, actual.len());
        assert_eq!(expected_object1, actual[0].object);
        assert_eq!(expected_object2, actual[1].object);
    }
}