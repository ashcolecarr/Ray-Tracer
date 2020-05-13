use super::EPSILON;
use super::intersection::Intersection;
use super::material::Material;
use super::matrix::Matrix;
use super::near_eq;
use super::ray::Ray;
use super::shape::Shape;
use super::tuple::Tuple;
use std::f64::INFINITY;
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Debug, Clone)]
pub struct Cube {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
    pub casts_shadow: bool,
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.transform == other.transform &&
            self.material == other.material && self.casts_shadow == other.casts_shadow
    }
}

impl Cube {
    pub fn new() -> Self {
        static ID_COUNT: AtomicI32 = AtomicI32::new(1);

        Self {
            id: ID_COUNT.fetch_add(1, Ordering::Relaxed),
            transform: Matrix::identity(4),
            material: Default::default(),
            casts_shadow: true,
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = Cube::check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = Cube::check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = Cube::check_axis(ray.origin.z, ray.direction.z);

        let tmin = vec![xtmin, ytmin, ztmin].iter().fold(0./0., |max, &n| f64::max(max, n));
        let tmax = vec![xtmax, ytmax, ztmax].iter().fold(0./0., |min, &n| f64::min(min, n));

        if tmin > tmax {
            return vec![];
        }

        vec![
            Intersection::new(tmin, Shape::Cube(self.clone())),
            Intersection::new(tmax, Shape::Cube(self.clone())),
        ]
    }

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1. - origin;
        let tmax_numerator = 1. - origin;

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

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let maxc = vec![world_point.x.abs(), world_point.y.abs(), world_point.z.abs()]
            .iter().fold(0./0., |max, &n| f64::max(max, n));
        
        if near_eq(maxc, world_point.x.abs()) {
            Tuple::vector(world_point.x, 0., 0.)
        } else if near_eq(maxc, world_point.y.abs()) {
            Tuple::vector(0., world_point.y, 0.)
        } else {
            Tuple::vector(0., 0., world_point.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::near_eq;
    use super::super::ray::Ray;
    use super::super::shape::{Shape, Actionable};
    use super::super::tuple::Tuple;

    #[test]
    fn ray_intersects_cube() {
        let cube = Shape::Cube(Cube::new());
        let rays = vec![
            Ray::new(Tuple::point(5., 0.5, 0.), Tuple::vector(-1., 0., 0.)),
            Ray::new(Tuple::point(-5., 0.5, 0.), Tuple::vector(1., 0., 0.)),
            Ray::new(Tuple::point(0.5, 5., 0.), Tuple::vector(0., -1., 0.)),
            Ray::new(Tuple::point(0.5, -5., 0.), Tuple::vector(0., 1., 0.)),
            Ray::new(Tuple::point(0.5, 0., 5.), Tuple::vector(0., 0., -1.)),
            Ray::new(Tuple::point(0.5, 0., -5.), Tuple::vector(0., 0., 1.)),
            Ray::new(Tuple::point(0., 0.5, 0.), Tuple::vector(0., 0., 1.)),
        ];

        let expected_count = 2;
        let expected_ts: Vec<(f64, f64)> = vec![(4., 6.), (4., 6.), (4., 6.), 
            (4., 6.), (4., 6.), (4., 6.), (-1., 1.)];
        
        for source in expected_ts.iter().zip(rays) {
            let (expected, ray) = source;
            let actual = cube.intersect(ray);
            
            assert_eq!(expected_count, actual.len());
            assert!(near_eq(expected.0, actual[0].t));
            assert!(near_eq(expected.1, actual[1].t));
        }
    }

    #[test]
    fn ray_misses_cube() {
        let cube = Shape::Cube(Cube::new());
        let rays = vec![
            Ray::new(Tuple::point(-2., 0., 0.), Tuple::vector(0.2673, 0.5345, 0.8018)),
            Ray::new(Tuple::point(0., -2., 0.), Tuple::vector(0.8018, 0.2673, 0.5345)),
            Ray::new(Tuple::point(0., 0., -2.), Tuple::vector(0.5345, 0.8018, 0.2673)),
            Ray::new(Tuple::point(2., 0., 2.), Tuple::vector(0., 0., -1.)),
            Ray::new(Tuple::point(0., 2., 2.), Tuple::vector(0., -1., 0.)),
            Ray::new(Tuple::point(2., 2., 0.), Tuple::vector(-1., 0., 0.)),
        ];

        for ray in rays {
            let actual = cube.intersect(ray);
            
            assert!(actual.is_empty());
        }
    }

    #[test]
    fn normal_on_surface_of_cube() {
        let cube = Shape::Cube(Cube::new());
        let points = vec![
            Tuple::point(1., 0.5, -0.8), 
            Tuple::point(-1., -0.2, 0.9),
            Tuple::point(-0.4, 1., -0.1), 
            Tuple::point(0.3, -1., -0.7),
            Tuple::point(-0.6, 0.3, 1.), 
            Tuple::point(0.4, 0.4, -1.),
            Tuple::point(1., 1., 1.), 
            Tuple::point(-1., -1., -1.), 
        ];

        let expecteds: Vec<Tuple> = vec![
            Tuple::vector(1., 0., 0.), 
            Tuple::vector(-1., 0., 0.),
            Tuple::vector(0., 1., 0.), 
            Tuple::vector(0., -1., 0.),
            Tuple::vector(0., 0., 1.), 
            Tuple::vector(0., 0., -1.),
            Tuple::vector(1., 0., 0.), 
            Tuple::vector(-1., 0., 0.), 
        ];
        
        for source in expecteds.iter().zip(points) {
            let (expected, point) = source;
            let actual = cube.normal_at(point);
            
            assert_eq!(*expected, actual);
        }
    }
}