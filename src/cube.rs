use super::EPSILON;
use super::intersection::Intersection;
use super::near_eq;
use super::ray::Ray;
use super::shape::{Shape, ShapeCommon};
use super::tuple::Tuple;
use std::f64::INFINITY;

#[derive(Debug)]
pub struct Cube {
    pub shape: Shape,
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape
    }
}

impl Cube {
    pub fn new() -> Self {
        Self {
            shape: Shape::new(),
        }
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
}

impl ShapeCommon for Cube {
    fn get_shape(&self) -> &Shape {
        &self.shape
    }

    fn get_shape_mut(&self) -> &mut Shape {
        &mut self.shape
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = Cube::check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = Cube::check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = Cube::check_axis(ray.origin.z, ray.direction.z);

        let tmin = vec![xtmin, ytmin, ztmin].iter().fold(0./0., |max, &n| f64::max(max, n));
        let tmax = vec![xtmax, ytmax, ztmax].iter().fold(0./0., |min, &n| f64::min(min, n));

        if tmin > tmax {
            return vec![];
        }

        vec![
            Intersection::new(tmin, self),
            Intersection::new(tmax, self),
        ]
    }

    fn local_normal_at(&self, world_point: Tuple) -> Tuple {
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
    use super::super::tuple::Tuple;

    #[test]
    fn ray_intersects_cube() {
        let cube = Cube::new();
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
        let cube = Cube::new();
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
        let cube = Cube::new();
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