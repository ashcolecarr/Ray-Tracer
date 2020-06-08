use super::bound::Bound;
use super::EPSILON;
use super::generate_object_id;
use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
use super::near_eq;
use super::ray::Ray;
use super::shape::Shape;
use super::tuple::Tuple;
use std::f64::INFINITY;
use std::mem::swap;

#[derive(Debug, Clone)]
pub struct Cylinder {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
    pub casts_shadow: bool,
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
    pub parent: Option<i32>,
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform &&
            self.material == other.material && self.casts_shadow == other.casts_shadow &&
            near_eq(self.minimum, other.minimum) && near_eq(self.maximum, other.maximum) &&
            self.closed == other.closed && self.parent == other.parent
    }
}

impl Cylinder {
    pub fn new() -> Self {
        Self {
            id: generate_object_id(),
            transform: Matrix::identity(4),
            material: Default::default(),
            casts_shadow: true,
            minimum: -INFINITY,
            maximum: INFINITY,
            closed: false,
            parent: None,
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];

        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);
        if !near_eq(a, 0.) {
            let b = 2. * ray.origin.x * ray.direction.x + 2. * ray.origin.z * ray.direction.z;
            let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.;

            let disc = b.powi(2) - 4. * a * c;
            if disc < 0. {
                return vec![];
            }

            let mut t0 = (-b - disc.sqrt()) / (2. * a);
            let mut t1 = (-b + disc.sqrt()) / (2. * a);
            if t0 > t1 {
                swap(&mut t0, &mut t1);
            }

            let y0 = ray.origin.y + t0 * ray.direction.y;
            if self.minimum < y0 && y0 < self.maximum {
                intersections.push(Intersection::new(t0, Shape::Cylinder(self.clone())));
            }

            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                intersections.push(Intersection::new(t1, Shape::Cylinder(self.clone())));
            }
        }

        self.intersect_caps(ray, &mut intersections);

        intersections
    }

    fn intersect_caps(&self, ray: Ray, intersections: &mut Vec<Intersection>) {
        if !self.closed || near_eq(ray.direction.y, 0.) {
            return;
        }

        let t_lower = (self.minimum - ray.origin.y) / ray.direction.y;
        if Cylinder::check_cap(ray, t_lower) {
            intersections.push(Intersection::new(t_lower, Shape::Cylinder(self.clone())));
        }

        let t_upper = (self.maximum - ray.origin.y) / ray.direction.y;
        if Cylinder::check_cap(ray, t_upper) {
            intersections.push(Intersection::new(t_upper, Shape::Cylinder(self.clone())));
        }
    } 
    
    fn check_cap(ray: Ray, t: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        
        let result = x.powi(2) + z.powi(2); 
        
        result < 1. || near_eq(result, 1.)
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let distance = world_point.x.powi(2) + world_point.z.powi(2);

        if distance < 1. && (world_point.y > self.maximum - EPSILON ||
            near_eq(world_point.y, self.maximum - EPSILON)) {

            Tuple::vector(0., 1., 0.)
        } else if distance < 1. && (world_point.y < self.minimum + EPSILON ||
            near_eq(world_point.y, self.minimum + EPSILON)) {

            Tuple::vector(0., -1., 0.)
        } else {
            Tuple::vector(world_point.x, 0., world_point.z)
        }
    }

    pub fn bounds_of(&self) -> Bound {
        Bound::bounding_box_init(Tuple::point(-1., self.minimum, -1.),
            Tuple::point(1., self.maximum, 1.))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::near_eq;
    use super::super::ORIGIN;
    use super::super::ray::Ray;
    use super::super::tuple::Tuple;
    use std::f64::INFINITY;

    #[test]
    fn ray_misses_cylinder() {
        let cylinder = Cylinder::new();
        let rays: Vec<Ray> = vec![
            Ray::new(Tuple::point(1., 0., 0.), Tuple::vector(0., 1., 0.).normalize()),
            Ray::new(ORIGIN, Tuple::vector(0., 1., 0.).normalize()),
            Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(1., 1., 1.).normalize()),
        ];

        for ray in rays {
            let actual = cylinder.intersect(ray);

            assert!(actual.is_empty());
        }
    }

    #[test]
    fn ray_strikes_cylinder() {
        let cylinder = Cylinder::new();
        let rays: Vec<Ray> = vec![
            Ray::new(Tuple::point(1., 0., -5.), Tuple::vector(0., 0., 1.).normalize()),
            Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.).normalize()),
            Ray::new(Tuple::point(0.5, 0., -5.), Tuple::vector(0.1, 1., 1.).normalize()),
        ];

        let expected_count = 2;
        let expecteds: Vec<(f64, f64)> = vec![(5., 5.), (4., 6.), (6.80798, 7.08872)];

        for source in expecteds.iter().zip(rays) {
            let (expected, ray) = source;
            let actual = cylinder.intersect(ray);

            assert_eq!(expected_count, actual.len());
            assert!(near_eq(expected.0, actual[0].t));
            assert!(near_eq(expected.1, actual[1].t));
        }
    }

    #[test]
    fn normal_vector_on_cylinder() {
        let cylinder = Cylinder::new();

        let points = vec![
            Tuple::point(1., 0., 0.), 
            Tuple::point(0., 5., -1.),
            Tuple::point(0., -2., 1.), 
            Tuple::point(-1., 1., 0.),
        ];

        let expecteds: Vec<Tuple> = vec![
            Tuple::vector(1., 0., 0.), 
            Tuple::vector(0., 0., -1.),
            Tuple::vector(0., 0., 1.), 
            Tuple::vector(-1., 0., 0.),
        ];

        for source in expecteds.iter().zip(points) {
            let (expected, point) = source;
            let actual = cylinder.normal_at(point);
            
            assert_eq!(*expected, actual);
        }
    }

    #[test]
    fn default_minimum_and_maximum_for_cylinder() {
        let expected_minimum = -INFINITY; 
        let expected_maximum = INFINITY;
        
        let actual = Cylinder::new();

        assert_eq!(expected_minimum, actual.minimum);
        assert_eq!(expected_maximum, actual.maximum);
    }

    #[test]
    fn intersecting_constrained_cylinder() {
        let mut cylinder = Cylinder::new();
        cylinder.minimum = 1.;
        cylinder.maximum = 2.;
        let rays = vec![
            Ray::new(Tuple::point(0., 1.5, 0.), Tuple::vector(0.1, 1., 0.).normalize()),
            Ray::new(Tuple::point(0., 3., -5.), Tuple::vector(0., 0., 1.).normalize()),
            Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.).normalize()),
            Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.).normalize()),
            Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.).normalize()),
            Ray::new(Tuple::point(0., 1.5, -2.), Tuple::vector(0., 0., 1.).normalize()),
        ];

        let expected_counts: Vec<usize> = vec![0, 0, 0, 0, 0, 2];
        
        for source in expected_counts.iter().zip(rays) {
            let (expected, ray) = source;
            let actual = cylinder.intersect(ray);
            
            assert_eq!(*expected, actual.len());
        }
    }

    #[test]
    fn default_closed_value_for_cylinder() {
        let cylinder = Cylinder::new();

        let expected = false;

        let actual = cylinder.closed;

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn intersecting_caps_of_closed_cylinder() {
        let mut cylinder = Cylinder::new();
        cylinder.minimum = 1.;
        cylinder.maximum = 2.;
        cylinder.closed = true;
        let rays = vec![
            Ray::new(Tuple::point(0., 3., 0.), Tuple::vector(0., -1., 0.).normalize()),
            Ray::new(Tuple::point(0., 3., -2.), Tuple::vector(0., -1., 2.).normalize()),
            Ray::new(Tuple::point(0., 4., -2.), Tuple::vector(0., -1., 1.).normalize()),
            Ray::new(Tuple::point(0., 0., -2.), Tuple::vector(0., 1., 2.).normalize()),
            Ray::new(Tuple::point(0., -1., -2.), Tuple::vector(0., 1., 1.).normalize()),
        ];

        let expected_counts: Vec<usize> = vec![2; 5];
        
        for source in expected_counts.iter().zip(rays) {
            let (expected, ray) = source;
            let actual = cylinder.intersect(ray);
            
            assert_eq!(*expected, actual.len());
        }
    }

    #[test]
    fn normal_vector_on_cylinder_end_caps() {
        let mut cylinder = Cylinder::new();
        cylinder.minimum = 1.;
        cylinder.maximum = 2.;
        cylinder.closed = true;

        let points = vec![
            Tuple::point(0., 1., 0.), 
            Tuple::point(0.5, 1., 0.),
            Tuple::point(0., 1., 0.5), 
            Tuple::point(0., 2., 0.),
            Tuple::point(0.5, 2., 0.),
            Tuple::point(0., 2., 0.5),
        ];

        let expecteds: Vec<Tuple> = vec![
            Tuple::vector(0., -1., 0.), 
            Tuple::vector(0., -1., 0.), 
            Tuple::vector(0., -1., 0.), 
            Tuple::vector(0., 1., 0.),
            Tuple::vector(0., 1., 0.),
            Tuple::vector(0., 1., 0.),
        ];

        for source in expecteds.iter().zip(points) {
            let (expected, point) = source;
            let actual = cylinder.normal_at(point);
            
            assert_eq!(*expected, actual);
        }
    }

    #[test]
    fn unbounded_cylinder_has_bounding_box() {
        let shape = Cylinder::new();
        let bounding_box = shape.bounds_of();

        let expected_minimum = Tuple::point(-1., -INFINITY, -1.);
        let expected_maximum = Tuple::point(1., INFINITY, 1.);

        let actual_minimum = bounding_box.minimum;
        let actual_maximum = bounding_box.maximum;

        assert!(near_eq(expected_minimum.x, actual_minimum.x));
        assert_eq!(expected_minimum.y, actual_minimum.y);
        assert!(near_eq(expected_minimum.z, actual_minimum.z));
        assert!(near_eq(expected_maximum.x, actual_maximum.x));
        assert_eq!(expected_maximum.y, actual_maximum.y);
        assert!(near_eq(expected_maximum.z, actual_maximum.z));
    }

    #[test]
    fn bounded_cylinder_has_bounding_box() {
        let mut shape = Cylinder::new();
        shape.minimum = -5.;
        shape.maximum = 3.;
        let bounding_box = shape.bounds_of();

        let expected_minimum = Tuple::point(-1., -5., -1.);
        let expected_maximum = Tuple::point(1., 3., 1.);

        let actual_minimum = bounding_box.minimum;
        let actual_maximum = bounding_box.maximum;

        assert!(near_eq(expected_minimum.x, actual_minimum.x));
        assert_eq!(expected_minimum.y, actual_minimum.y);
        assert!(near_eq(expected_minimum.z, actual_minimum.z));
        assert!(near_eq(expected_maximum.x, actual_maximum.x));
        assert_eq!(expected_maximum.y, actual_maximum.y);
        assert!(near_eq(expected_maximum.z, actual_maximum.z));
    }
}