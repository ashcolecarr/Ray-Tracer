use super::bound::Bound;
use super::EPSILON;
use super::generate_object_id;
use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
use super::ray::Ray;
use super::shape::Shape;
use super::tuple::Tuple;
use std::f64::INFINITY;

#[derive(Debug, Clone)]
pub struct Plane {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
    pub casts_shadow: bool,
    pub parent: Option<i32>,
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform && 
            self.material == other.material && self.casts_shadow == other.casts_shadow &&
            self.parent == other.parent
    }
}

impl Plane {
    pub fn new() -> Self {
        Self {
            id: generate_object_id(),
            transform: Matrix::identity(4),
            material: Default::default(),
            casts_shadow: true,
            parent: None,
        }
    }
    
    pub fn get_id(&self) -> &i32 {
        &self.id
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

    pub fn bounds_of(&self) -> Bound {
        Bound::bounding_box_init(Tuple::point(-INFINITY, 0., -INFINITY),
            Tuple::point(INFINITY, 0., INFINITY))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::near_eq;
    use super::super::ORIGIN;
    use super::super::ray::Ray;
    use super::super::shape::Shape;
    use super::super::tuple::Tuple;
    use std::f64::INFINITY;

    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let plane = Plane::new();

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
        let plane = Plane::new();
        let ray = Ray::new(Tuple::point(0., 10., 0.), Tuple::vector(0., 0., 1.));

        let actual = plane.intersect(ray);

        assert!(actual.is_empty());
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let plane = Plane::new();
        let ray = Ray::new(ORIGIN, Tuple::vector(0., 0., 1.));

        let actual = plane.intersect(ray);

        assert!(actual.is_empty());
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let plane = Plane::new();
        let ray = Ray::new(Tuple::point(0., 1., 0.), Tuple::vector(0., -1., 0.));

        let expected_count = 1;
        let expected_t = 1.;
        let expected_object = plane.clone();

        let actual = plane.intersect(ray);
        let actual_object = if let Shape::Plane(plane) = actual[0].object.clone() { plane } else { panic!("") };

        assert_eq!(expected_count, actual.len());
        assert_eq!(expected_t, actual[0].t);
        assert_eq!(expected_object, actual_object);
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let plane = Plane::new();
        let ray = Ray::new(Tuple::point(0., -1., 0.), Tuple::vector(0., 1., 0.));

        let expected_count = 1;
        let expected_t = 1.;
        let expected_object = plane.clone();

        let actual = plane.intersect(ray);
        let actual_object = if let Shape::Plane(plane) = actual[0].object.clone() { plane } else { panic!("") };

        assert_eq!(expected_count, actual.len());
        assert_eq!(expected_t, actual[0].t);
        assert_eq!(expected_object, actual_object);
    }

    #[test]
    fn plane_has_bounding_box() {
        let shape = Plane::new();
        let bounding_box = shape.bounds_of();

        let expected_minimum = Tuple::point(-INFINITY, 0., -INFINITY);
        let expected_maximum = Tuple::point(INFINITY, 0., INFINITY);

        let actual_minimum = bounding_box.minimum;
        let actual_maximum = bounding_box.maximum;

        assert_eq!(expected_minimum.x, actual_minimum.x);
        assert!(near_eq(expected_minimum.y, actual_minimum.y));
        assert_eq!(expected_minimum.z, actual_minimum.z);
        assert_eq!(expected_maximum.x, actual_maximum.x);
        assert!(near_eq(expected_maximum.y, actual_maximum.y));
        assert_eq!(expected_maximum.z, actual_maximum.z);
    }
}