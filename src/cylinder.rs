use super::EPSILON;
use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
use super::near_eq;
use super::ray::Ray;
use super::shape::Shape;
use super::tuple::Tuple;
use std::f64::INFINITY;
use std::mem::swap;
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Debug, Clone)]
pub struct Cylinder {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
    pub casts_shadow: bool,
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform &&
            self.material == other.material && self.casts_shadow == other.casts_shadow &&
            self.minimum == other.minimum && self.maximum == other.maximum &&
            self.closed == other.closed
    }
}

impl Cylinder {
    pub fn new() -> Self {
        static ID_COUNT: AtomicI32 = AtomicI32::new(1);

        Self {
            id: ID_COUNT.fetch_add(1, Ordering::Relaxed),
            transform: Matrix::identity(4),
            material: Default::default(),
            casts_shadow: true,
            minimum: -INFINITY,
            maximum: INFINITY,
            closed: false,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::near_eq;
    use super::super::ORIGIN;
    use super::super::ray::Ray;
    use super::super::shape::{Shape, Actionable};
    use super::super::tuple::Tuple;
    use std::f64::INFINITY;

    #[test]
    fn ray_misses_cylinder() {
        let cylinder = Shape::Cylinder(Cylinder::new());
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
        let cylinder = Shape::Cylinder(Cylinder::new());
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
        let cylinder = Shape::Cylinder(Cylinder::new());

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
        
        let actual = Shape::Cylinder(Cylinder::new());

        assert_eq!(expected_minimum, actual.get_minimum());
        assert_eq!(expected_maximum, actual.get_maximum());
    }

    #[test]
    fn intersecting_constrained_cylinder() {
        let mut cylinder = Shape::Cylinder(Cylinder::new());
        cylinder.set_minimum(1.);
        cylinder.set_maximum(2.);
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
        let cylinder = Shape::Cylinder(Cylinder::new());

        let expected = false;

        let actual = cylinder.get_closed();

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn intersecting_caps_of_closed_cylinder() {
        let mut cylinder = Shape::Cylinder(Cylinder::new());
        cylinder.set_minimum(1.);
        cylinder.set_maximum(2.);
        cylinder.set_closed(true);
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
        let mut cylinder = Shape::Cylinder(Cylinder::new());
        cylinder.set_minimum(1.);
        cylinder.set_maximum(2.);
        cylinder.set_closed(true);

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
}