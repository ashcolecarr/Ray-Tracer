use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
use super::ORIGIN;
use super::ray::Ray;
use super::tuple::Tuple;
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Debug, Clone)]
pub struct Sphere {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform &&
            self.material == other.material
    }
}

impl Sphere {
    pub fn new() -> Self {
        static ID_COUNT:AtomicI32 = AtomicI32::new(1);

        Self {
            id: ID_COUNT.fetch_add(1, Ordering::Relaxed),
            transform: Matrix::identity(4),
            material: Default::default(),
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let ray_transform = match self.transform.inverse() {
            Some(i) => ray.transform(i),
            None => ray,
        };
        let sphere_to_ray = ray_transform.origin - ORIGIN;

        let a = ray_transform.direction.dot(ray_transform.direction);
        let b = 2. * ray_transform.direction.dot(sphere_to_ray);
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

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let inverse = self.transform.inverse().unwrap();
        let object_point = inverse.clone() * world_point;
        let object_normal = object_point - ORIGIN;

        let mut world_normal = inverse.transpose() * object_normal;
        world_normal.w = 0.;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::material::Material;
    use super::super::matrix::Matrix;
    use super::super::near_eq;
    use super::super::ORIGIN;
    use super::super::ray::Ray;
    use super::super::transformation::*;
    use super::super::tuple::Tuple;
    use std::f64::consts::PI;

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
        let ray = Ray::new(ORIGIN, Tuple::vector(0., 0., 1.));
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

    #[test]
    fn sphere_default_transformation() {
        let sphere = Sphere::new();

        let expected = Matrix::identity(4);

        let actual = sphere.transform;

        assert_eq!(expected, actual);
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut sphere = Sphere::new();
        let transform =  translate(2., 3., 4.);

        let expected = transform.clone();

        sphere.set_transform(transform);
        let actual = sphere.transform;

        assert_eq!(expected, actual);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut sphere = Sphere::new();
        sphere.set_transform(scale(2., 2., 2.));

        let expected_count = 2;
        let expected_t1 = 3.;
        let expected_t2 = 7.;

        let actual = sphere.intersect(ray);

        assert_eq!(expected_count, actual.len());
        assert_eq!(expected_t1, actual[0].t);
        assert_eq!(expected_t2, actual[1].t);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let ray = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut sphere = Sphere::new();
        sphere.set_transform(translate(5., 0., 0.));

        let actual = sphere.intersect(ray);

        assert!(actual.is_empty());
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let sphere = Sphere::new();
        
        let expected = Tuple::vector(1., 0., 0.);

        let actual = sphere.normal_at(Tuple::point(1., 0., 0.));

        assert_eq!(expected, actual);
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let sphere = Sphere::new();
        
        let expected = Tuple::vector(0., 1., 0.);

        let actual = sphere.normal_at(Tuple::point(0., 1., 0.));

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let sphere = Sphere::new();
        
        let expected = Tuple::vector(0., 0., 1.);

        let actual = sphere.normal_at(Tuple::point(0., 0., 1.));

        assert_eq!(expected, actual);
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let value = 3_f64.sqrt() / 3.;
        let sphere = Sphere::new();

        let expected = Tuple::vector(value, value, value);

        let actual = sphere.normal_at(Tuple::point(value, value, value));
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn normal_is_normalized_vector() {
        let value = 3_f64.sqrt() / 3.;
        let sphere = Sphere::new();

        let actual = sphere.normal_at(Tuple::point(value, value, value));

        let expected = actual.normalize();
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let mut sphere = Sphere::new();
        sphere.set_transform(translate(0., 1., 0.));

        let expected = Tuple::vector(0., 0.70711, -0.70711);

        let actual = sphere.normal_at(Tuple::point(0., 1.70711, -0.70711));

        assert_eq!(expected, actual);
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut sphere = Sphere::new();
        let transform = scale(1., 0.5, 1.) * rotate(PI / 5., Axis::Z);
        sphere.set_transform(transform);

        let expected = Tuple::vector(0., 0.97014, -0.24254);

        let actual = sphere.normal_at(Tuple::point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.));

        assert_eq!(expected, actual);
    }

    #[test]
    fn sphere_has_default_material() {
        let sphere = Sphere::new();

        let expected: Material = Default::default();

        let actual = sphere.material;

        assert_eq!(expected, actual);
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let mut sphere = Sphere::new();
        let mut material: Material = Default::default();
        material.ambient = 1.;
        sphere.material = material;

        let expected = material;

        let actual = sphere.material;

        assert_eq!(expected, actual);
    }
}