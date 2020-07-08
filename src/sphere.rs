use super::bound::Bound;
use super::generate_object_id;
use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
use super::ORIGIN;
use super::ray::Ray;
use super::shape::Shape;
use super::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Sphere {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
    pub casts_shadow: bool,
    pub parent: Option<i32>,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform && 
            self.material == other.material && self.casts_shadow == other.casts_shadow &&
            self.parent == other.parent
    }
}

impl Sphere {
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
        let sphere_to_ray = ray.origin - ORIGIN;

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
            Intersection::new(t1, Shape::Sphere(self.clone())),
            Intersection::new(t2, Shape::Sphere(self.clone())),
        ]
    }

    pub fn normal_at(&self, world_point: Tuple, _hit: Intersection) -> Tuple {
        world_point - ORIGIN
    }

    pub fn glass_sphere() -> Self {
        let mut material: Material = Default::default();
        material.transparency = 1.;
        material.refractive_index = 1.5;

        let mut sphere = Sphere::new();
        sphere.material = material;

        sphere
    }

    pub fn bounds_of(&self) -> Bound {
        Bound::bounding_box_init(Tuple::point(-1., -1., -1.),
            Tuple::point(1., 1., 1.))
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
        let actual_object1 = if let Shape::Sphere(sphere) = actual[0].object.clone() { sphere } else { panic!("") };
        let actual_object2 = if let Shape::Sphere(sphere) = actual[1].object.clone() { sphere } else { panic!("") };

        assert_eq!(expected_count, actual.len());
        assert_eq!(expected_object1, actual_object1);
        assert_eq!(expected_object2, actual_object2);
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let sphere = Sphere::new();
        let intersection = Intersection::new(1., Shape::Sphere(sphere.clone()));
        
        let expected = Tuple::vector(1., 0., 0.);

        let actual = sphere.normal_at(Tuple::point(1., 0., 0.), intersection);

        assert_eq!(expected, actual);
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let sphere = Sphere::new();
        let intersection = Intersection::new(1., Shape::Sphere(sphere.clone()));
        
        let expected = Tuple::vector(0., 1., 0.);

        let actual = sphere.normal_at(Tuple::point(0., 1., 0.), intersection);

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let sphere = Sphere::new();
        let intersection = Intersection::new(1., Shape::Sphere(sphere.clone()));
        
        let expected = Tuple::vector(0., 0., 1.);

        let actual = sphere.normal_at(Tuple::point(0., 0., 1.), intersection);

        assert_eq!(expected, actual);
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let value = 3_f64.sqrt() / 3.;
        let sphere = Sphere::new();
        let intersection = Intersection::new(1., Shape::Sphere(sphere.clone()));

        let expected = Tuple::vector(value, value, value);

        let actual = sphere.normal_at(Tuple::point(value, value, value), intersection);
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn normal_is_normalized_vector() {
        let value = 3_f64.sqrt() / 3.;
        let sphere = Sphere::new();
        let intersection = Intersection::new(1., Shape::Sphere(sphere.clone()));

        let actual = sphere.normal_at(Tuple::point(value, value, value), intersection);

        let expected = actual.normalize();
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn helper_for_producing_sphere_with_glassy_material() {
        let expected_transform = Matrix::identity(4);
        let expected_transparency = 1.;
        let expected_refractive_index = 1.5;

        let actual = Sphere::glass_sphere();

        assert_eq!(expected_transform, actual.transform);
        assert_eq!(expected_transparency, actual.material.transparency);
        assert_eq!(expected_refractive_index, actual.material.refractive_index);
    }

    #[test]
    fn sphere_has_bounding_box() {
        let shape = Sphere::new();
        let bounding_box = shape.bounds_of();

        let expected_minimum = Tuple::point(-1., -1., -1.);
        let expected_maximum = Tuple::point(1., 1., 1.);

        let actual_minimum = bounding_box.minimum;
        let actual_maximum = bounding_box.maximum;

        assert_eq!(expected_minimum, actual_minimum);
        assert_eq!(expected_maximum, actual_maximum);
    }
}